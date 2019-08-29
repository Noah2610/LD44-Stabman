mod savefile;

use std::collections::HashMap;

use amethyst::audio::{output::Output, AudioSink, Source};
use amethyst::ecs::{Entities, Join, ReadStorage, WriteStorage};

use climer::Time;

use super::super::state_prelude::*;
use super::level_loader::LevelLoader;
use savefile::TimeData;

const TIMER_Z: f32 = 10.0;

pub mod prelude {
    pub use super::LevelManager;
}

pub struct LevelManager {
    pub settings:          SettingsLevelManagerCampaign,
    pub level_index:       usize,
    pub has_won_game:      bool,
    player_checkpoint_opt: Option<Player>,
    completed_levels:      Vec<String>,
    level_times:           HashMap<String, TimeData>,
    global_time:           Option<TimeData>,
    current_song:          Option<String>,
}

impl LevelManager {
    pub fn new(
        data: &mut StateData<CustomGameData<CustomData>>,
        settings: SettingsLevelManagerCampaign,
        new_game: bool,
    ) -> Self {
        let mut level_manager = Self {
            settings:              settings,
            level_index:           0,
            has_won_game:          false,
            player_checkpoint_opt: None,
            completed_levels:      Vec::new(),
            level_times:           HashMap::new(),
            global_time:           None,
            current_song:          None,
        };
        level_manager.load_from_savefile(data);
        if new_game {
            level_manager.remove_save(data);
        }
        level_manager
    }

    pub fn load_current_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        // Set LoadingLevel resource to `true`, to let the LoaderSystem know
        // that it should stop running.
        data.world.write_resource::<LoadingLevel>().0 = true;

        // First, remove all existing entities, which do not have `DontDeleteOnNextLevel`.
        data.world.exec(
            |(entities, dont_deletes, players): (
                Entities,
                ReadStorage<DontDeleteOnNextLevel>,
                ReadStorage<Player>,
            )| {
                for (entity, _) in (&entities, !&dont_deletes).join() {
                    entities.delete(entity).unwrap();
                }
            },
        );
        data.world.maintain();

        let current_level_name = self.level_name();
        let level_filepath = resource(format!(
            "{}/{}",
            self.settings.levels_dir, current_level_name
        ));

        let mut level_loader = LevelLoader::new(self.settings.clone());
        level_loader.load_level(level_filepath);
        level_loader.build(data);

        // Load checkpoint / player data
        if let Some(player_checkpoint) = &self.player_checkpoint_opt {
            data.world.exec(|mut players: WriteStorage<Player>| {
                if let Some(player) = (&mut players).join().next() {
                    *player = player_checkpoint.clone();
                }
            });
        }

        // Start all turret timers
        data.world.exec(|mut enemy_ais: WriteStorage<EnemyAi>| {
            (&mut enemy_ais).join().for_each(|enemy_ai| {
                if let EnemyAi::Turret(ai_data) = enemy_ai {
                    ai_data.shot_timer.start().unwrap();
                }
            });
        });

        {
            // (Re)start level timer
            let mut timers = data.world.write_resource::<Timers>();
            timers.level.start().unwrap();
            // Resume global timer
            timers.global.as_mut().map(|timer| {
                if timer.state.is_paused() {
                    timer.resume().unwrap();
                }
            });
            // Start global timer, if this is the first level
            if self.is_first_level() {
                timers.global = Some(Timer::default());
                timers.global.as_mut().map(|timer| timer.start().unwrap());
            }
        }

        // Create timer UI (if level has been completed before)
        if self.has_completed_current_level() {
            create_timer_ui(
                TimerType::Level,
                &self.settings.level_timer_ui,
                {
                    if let Some(times) =
                        self.level_times.get(&self.level_name())
                    {
                        Some(
                            if times.general < times.first {
                                times.general.clone()
                            } else {
                                times.first.clone()
                            },
                        )
                    } else {
                        None
                    }
                },
                data,
            );
        }
        if self.has_completed_game()
            && data.world.read_resource::<Timers>().global.is_some()
        {
            create_timer_ui(
                TimerType::Global,
                &self.settings.global_timer_ui,
                {
                    if let Some(times) = self.global_time.as_ref() {
                        Some(
                            if times.general < times.first {
                                times.general.clone()
                            } else {
                                times.first.clone()
                            },
                        )
                    } else {
                        None
                    }
                },
                data,
            );
        }

        // Set the CurrentLevelName resource
        data.world.write_resource::<CurrentLevelName>().0 =
            Some(self.level_name());

        // Set LoadingLevel resource to `false` again.
        data.world.write_resource::<LoadingLevel>().0 = false;
    }

    pub fn is_first_level(&self) -> bool {
        self.level_index == 0
    }

    pub fn update(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        // Check if the level was beaten and if the player has died
        let (player_in_goal, next_level, player_dead) = data.world.exec(
            |(goals, players, animations_containers, invincibles): (
                ReadStorage<Goal>,
                ReadStorage<Player>,
                ReadStorage<AnimationsContainer>,
                ReadStorage<Invincible>,
            )| {
                let player_in_goal = (&goals)
                    .join()
                    .find_map(|goal| Some(goal.next_level))
                    .unwrap_or(false);

                let next_level = player_in_goal
                    && (&players, &animations_containers)
                        .join()
                        .find_map(|(_, animations_container)| {
                            Some(animations_container.play_once.is_none())
                        })
                        .unwrap_or(false);

                let player_dead =
                    (&players, &animations_containers, !&invincibles)
                        .join()
                        .find_map(|(player, animations_container, _)| {
                            Some(
                                player.is_dead(), // && animations_container.play_once.is_none(),
                            )
                        })
                        .unwrap_or(false);

                (player_in_goal, next_level, player_dead)
            },
        );

        let level_name = self.level_name();
        let is_first_loop = self.is_first_loop(data);

        if player_in_goal {
            // Stop level timer
            let mut timers = data.world.write_resource::<Timers>();
            timers.level.finish().unwrap();
            let time = timers.level.time_output();
            println!("LEVEL TIME: {}", &time); // TODO
            let time_entry = self
                .level_times
                .entry(level_name.clone())
                .or_insert(TimeData {
                    general: time,
                    first:   time,
                });
            if time < time_entry.general {
                time_entry.general = time;
            }
            if is_first_loop && time < time_entry.first {
                time_entry.first = time;
            }
            // Pause global timer
            timers.global.as_mut().map(|timer| {
                if timer.state.is_running() {
                    timer.pause().unwrap()
                }
            });
        }

        if next_level {
            if !self.completed_levels.contains(&level_name) {
                self.completed_levels.push(level_name);
            }

            if self.has_next_level() {
                self.set_player_checkpoint(data);
                self.load_next_level(data, true);
            } else {
                self.win_game(data);
            }
        } else if player_dead {
            self.player_died(data);
        }
        self.play_current_song(data);
    }

    fn win_game(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        println!("You win!"); // TODO

        // Stop global timer
        {
            let is_first_loop = self.is_first_loop(data);
            let mut timers = data.world.write_resource::<Timers>();

            if let Some(global_timer) = timers.global.as_mut() {
                global_timer.finish().unwrap();
                let time = global_timer.time_output();
                println!("GLOBAL TIME: {}", &time); // TODO
                if self.global_time.is_none() {
                    self.global_time = Some(TimeData {
                        general: time,
                        first:   time,
                    });
                }
                self.global_time.as_mut().map(|global_time| {
                    if time < global_time.general {
                        global_time.general = time;
                    }
                    if is_first_loop && time < global_time.first {
                        global_time.first = time;
                    }
                });
            }
        }

        // Reset current death counters from stats,
        // and increase wins counter.
        {
            let mut stats = data.world.write_resource::<Stats>();
            stats.levels.reset_current_stats();
            stats.wins += 1;
        }
        // Continue game from the first level
        self.level_index = 0;
        self.set_player_checkpoint(data);
        self.save_to_savefile(data);

        self.has_won_game = true;

        // TODO
        // self.load_current_level(data);
        // Force update `HealthDisplay`
        // data.world.write_resource::<UpdateHealthDisplay>().0 = true;

        // Start the global timer again
        // data.world
        //     .write_resource::<Timers>()
        //     .global
        //     .as_mut()
        //     .map(|timer| timer.start().unwrap());
    }

    fn player_died(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        // Increase Stats death counter for the level
        data.world
            .write_resource::<Stats>()
            .level_mut(self.level_name())
            .deaths
            .increase();

        // Restart level and load player from checkoint
        self.restart_level(data);
        data.world.maintain();

        let health_increase = self.settings.health_increase_on_death;
        if health_increase > 0 {
            data.world.exec(|mut players: WriteStorage<Player>| {
                if let Some(player) = (&mut players).join().next() {
                    // Increase player's health
                    player.add_health(health_increase);
                    // Set player checkpoint
                    self.player_checkpoint_opt = Some(player.clone());
                }
            });
        }

        data.world.maintain();
        self.save_to_savefile(data);
    }

    fn play_current_song(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        if !data.world.read_resource::<AudioSink>().empty()
            && !self.should_play_current_song()
        {
            return;
        }

        let music_volume = data.world.settings().music_volume;
        let output = data.world.read_resource::<Output>();
        let mut sink = data.world.write_resource::<AudioSink>();
        sink.stop();
        *sink = AudioSink::new(&output);
        sink.set_volume(music_volume);

        let asset = data.world.read_resource::<AssetStorage<Source>>();
        let name = self.current_song_name();
        let handle = data
            .world
            .write_resource::<AudioHandles>()
            .get_or_load(resource(format!("audio/{}", name)), &data.world);
        if let Some(sound) = asset.get(&handle) {
            sink.append(sound).unwrap();
            self.current_song = Some(name.to_string());
        }
    }

    fn should_play_current_song(&self) -> bool {
        match &self.current_song {
            None => true,
            Some(name) => name.as_str() != self.current_song_name(),
        }
    }

    fn current_song_name(&self) -> &str {
        self.settings
            .song_names
            .get(self.level_index)
            .expect(&format!(
                "Song name at index {} doesn't exist",
                self.level_index
            ))
    }

    fn set_player_checkpoint(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        self.player_checkpoint_opt =
            data.world.exec(|players: ReadStorage<Player>| {
                (&players).join().next().map(Clone::clone)
            });
    }

    fn load_next_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
        should_save: bool,
    ) {
        if self.has_next_level() {
            self.level_index += 1;
            if should_save {
                // Save to savefile after changing the level_index but before loading the next level.
                self.save_to_savefile(data);
            }
            self.load_current_level(data);
        } else {
            panic!("There is no next level");
        }
    }

    fn has_next_level(&self) -> bool {
        self.level_index + 1 < self.settings.level_names.len()
    }

    fn restart_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        self.load_current_level(data);
    }

    fn has_completed_current_level(&self) -> bool {
        self.completed_levels.contains(&self.level_name())
    }

    fn has_completed_game(&self) -> bool {
        self.completed_levels.len() >= self.settings.level_names.len()
    }

    fn is_first_loop(
        &self,
        data: &StateData<CustomGameData<CustomData>>,
    ) -> bool {
        let stats = data.world.read_resource::<Stats>();
        stats.wins == 0
    }

    fn level_name(&self) -> String {
        self.settings
            .level_names
            .get(self.level_index)
            .expect(&format!(
                "Level at index {} doesn't exist",
                self.level_index
            ))
            .to_string()
    }

    fn level_index_from_name<T>(&self, level_name: T) -> usize
    where
        T: ToString,
    {
        let level_name = level_name.to_string();
        self.settings
            .level_names
            .iter()
            .enumerate()
            .find(|(_, name)| *name == &level_name)
            .expect(&format!("Level with name '{}' doesn't exist", level_name))
            .0
    }

    fn save_to_savefile(&self, data: &StateData<CustomGameData<CustomData>>) {
        let savefile_path = self.savefile_path();

        let savefile_data = savefile::SavefileData {
            player: self.player_checkpoint_opt.clone(),
            levels: savefile::LevelsData {
                current:     self.level_name(),
                completed:   self.completed_levels.clone(),
                times:       self.level_times.clone(),
                global_time: self.global_time.clone(),
            },
            stats:  Some(data.world.read_resource::<Stats>().clone()),
        };

        match serde_json::to_string(&savefile_data) {
            Ok(serialized) => {
                if crate::in_development_mode() {
                    // Write un-encrypted savefile.json
                    write_file(savefile_path, serialized).unwrap();
                } else {
                    // Write encrypted savefile.json
                    match encrypt(serialized) {
                        Ok(encrypted) => {
                            write_file(savefile_path, encrypted).unwrap()
                        }
                        Err(err) => eprintln!(
                            "An error occured while encrypting savefile: {}",
                            err
                        ),
                    }
                }
            }
            Err(err) => eprintln!(
                "Couldn't save savefile data to file, an error occured while \
                 serializing save data:\n{:#?}",
                err
            ),
        }
    }

    fn load_from_savefile(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        let savefile_path = self.savefile_path();
        if let Ok(raw) = read_file(savefile_path) {
            let mut retry = true;
            let mut tried_unencrypted = false;
            let mut json_raw = match decrypt(&raw) {
                Ok(s) => s,
                Err(err) => {
                    eprintln!(
                        "An error occured while decrypting savefile: \
                         {}\nTrying to load savefile without decrypting...",
                        err
                    );
                    tried_unencrypted = true;
                    raw.clone()
                }
            };

            while retry {
                match serde_json::from_str::<savefile::SavefileData>(&json_raw)
                {
                    Ok(deserialized) => {
                        retry = false;
                        self.player_checkpoint_opt = deserialized.player;
                        self.level_index = self
                            .level_index_from_name(deserialized.levels.current);
                        self.completed_levels = deserialized.levels.completed;
                        self.level_times = deserialized.levels.times;
                        self.global_time = deserialized.levels.global_time;
                        if let Some(stats) = deserialized.stats {
                            *data.world.write_resource::<Stats>() = stats;
                        }
                        eprintln!("Successfully loaded savefile!");
                    }
                    Err(err) => {
                        if tried_unencrypted {
                            retry = false;
                            eprintln!(
                                "Couldn't load savefile data from file, an \
                                 error occured while deserializing save \
                                 data:\n{:#?}",
                                err
                            );
                        } else {
                            tried_unencrypted = true;
                            json_raw = raw.clone();
                        }
                    }
                }
            }
        }
    }

    fn savefile_path(&self) -> String {
        use amethyst::utils::application_root_dir;
        format!("{}/{}", application_root_dir(), self.settings.savefile_path)
    }

    /// This method removes some data from the savefile, for starting a "new game".
    /// This does not delete stuff like stats, or best times.
    // TODO: Decide if this should really work this way,
    //       or if it should actually remove _everything_.
    fn remove_save(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        self.level_index = 0;
        self.player_checkpoint_opt = None;
        self.completed_levels = Vec::new();
        let mut stats = data.world.write_resource::<Stats>();
        stats.levels.reset_current_stats();
        stats.wins = 0;
    }
}

fn create_timer_ui(
    timer_type: TimerType,
    ui_settings: &crate::settings::SettingsTimerUi,
    highscore_opt: Option<Time>,
    data: &mut StateData<CustomGameData<CustomData>>,
) {
    let world = &mut data.world;

    let font = get_font(&ui_settings.font_file, &world);

    let size = (512.0, 32.0);
    let pos = (
        size.0 * 0.5 + ui_settings.offset.0,
        -size.1 * 0.5 + ui_settings.offset.1,
        TIMER_Z,
    );

    let ui_transform_name = match timer_type {
        TimerType::Level => "level_timer",
        TimerType::Global => "global_timer",
    };
    let ui_transform = new_ui_transform(
        ui_transform_name,
        AmethystAnchor::TopLeft,
        (pos.0, pos.1, pos.2, size.0, size.1, 0),
    );

    let mut ui_text = UiText::new(
        font,
        String::new(),
        ui_settings.font_color,
        ui_settings.font_size,
    );
    ui_text.align = AmethystAnchor::TopLeft;

    let timer_ui = TimerUi {
        timer_type,
        text_prefix: ui_settings.text_prefix.clone(),
    };

    world
        .create_entity()
        .with(timer_ui)
        .with(ui_transform)
        .with(ui_text)
        .build();

    if let Some(pb) = highscore_opt {
        let pb_settings = &ui_settings.highscore;

        let font = get_font(&pb_settings.font_file, &world);

        let pos = (
            pos.0 + pb_settings.offset.0,
            pos.1 + pb_settings.offset.1,
            TIMER_Z,
        );

        let ui_transform_name = &format!("{}_pb", ui_transform_name);
        let ui_transform = new_ui_transform(
            ui_transform_name,
            AmethystAnchor::TopLeft,
            (pos.0, pos.1, pos.2, size.0, size.1, 0),
        );

        let mut ui_text = UiText::new(
            font,
            format!("{}{}", pb_settings.text_prefix, pb),
            pb_settings.font_color,
            pb_settings.font_size,
        );
        ui_text.align = AmethystAnchor::TopLeft;

        world
            .create_entity()
            .with(ui_transform)
            .with(ui_text)
            .build();
    }
}

fn get_font<T>(font: T, world: &World) -> amethyst::ui::FontHandle
where
    T: ToString,
{
    world.read_resource::<AssetLoader>().load(
        resource(font),
        TtfFormat,
        Default::default(),
        (),
        &world.read_resource(),
    )
}

fn encrypt<T>(raw: T) -> Result<String, String>
where
    T: ToString,
{
    use base64::encode;

    Ok(encode(&raw.to_string()))
}

fn decrypt<T>(raw: T) -> Result<String, String>
where
    T: ToString,
{
    use base64::decode;

    match decode(&raw.to_string()) {
        Ok(decoded) => {
            String::from_utf8(decoded).map_err(|err| err.to_string())
        }
        Err(err) => Err(format!("Error decoding base64: {}", err)),
    }
}
