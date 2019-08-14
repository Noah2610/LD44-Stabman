mod savefile;

use amethyst::audio::{output::Output, AudioSink, Source};
use amethyst::ecs::{Entities, Join, ReadStorage, WriteStorage};

use super::super::state_prelude::*;
use super::level_loader::LevelLoader;

pub mod prelude {
    pub use super::LevelManager;
}

pub struct LevelManager {
    pub settings:          SettingsLevelManager,
    pub level_index:       usize,
    completed_levels:      Vec<String>,
    player_checkpoint_opt: Option<Player>,
}

impl LevelManager {
    pub fn new(settings: SettingsLevelManager) -> Self {
        let mut level_manager = Self {
            settings,
            level_index: 0,
            completed_levels: Vec::new(),
            player_checkpoint_opt: None,
        };
        level_manager.load_from_file();
        level_manager
    }

    pub fn load_current_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
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

        // (Re)start level timer
        {
            let mut timers = data.world.write_resource::<Timers>();
            timers.level.start().unwrap();
        }

        // Create timer UI (if level has been completed before)
        if self.has_completed_current_level() {
            self.create_timer_ui(
                TimerType::Level,
                &self.settings.level_timer_ui,
                "level_timer",
                data,
            );
        }
        if self.has_completed_game() {
            self.create_timer_ui(
                TimerType::Global,
                &self.settings.global_timer_ui,
                "global_timer",
                data,
            );
        }
    }

    pub fn update(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        // Check if the level was beaten
        let (next_level, player_dead) = data.world.exec(
            |(goals, players, animations_containers, invincibles): (
                ReadStorage<Goal>,
                ReadStorage<Player>,
                ReadStorage<AnimationsContainer>,
                ReadStorage<Invincible>,
            )| {
                let next_level = (&goals)
                    .join()
                    .find_map(|goal| Some(goal.next_level))
                    .unwrap_or(false);
                // && (&players, &animations_containers)
                //     .join()
                //     .find_map(|(_, animations_container)| {
                //         Some(animations_container.play_once.is_none())
                //     })
                // .unwrap_or(false);

                let player_dead =
                    (&players, &animations_containers, !&invincibles)
                        .join()
                        .find_map(|(player, animations_container, _)| {
                            Some(
                                player.is_dead(), // && animations_container.play_once.is_none(),
                            )
                        })
                        .unwrap_or(false);

                (next_level, player_dead)
            },
        );
        if next_level {
            {
                let mut timers = data.world.write_resource::<Timers>();
                timers.level.finish().unwrap();
                println!("LEVEL TIME: {}", timers.level.time_output());
            }
            let level_name = self.level_name();
            if !self.completed_levels.contains(&level_name) {
                self.completed_levels.push(self.level_name());
            }

            if self.has_next_level() {
                self.set_player_checkpoint(data);
                self.load_next_level(data, true);
                self.play_current_song(data);
            } else {
                // TODO: Beat game!
                println!("You win!");
                let mut timers = data.world.write_resource::<Timers>();
                timers.global.finish().unwrap();
                println!("GLOBAL TIME: {}", timers.global.time_output());
            }
        } else if player_dead {
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
            self.save_to_savefile();
        }
        if data.world.read_resource::<AudioSink>().empty() {
            self.play_current_song(data);
        }
    }

    fn play_current_song(
        &self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        let music_volume = data.world.settings().music_volume;
        let output = data.world.read_resource::<Output>();
        let mut sink = data.world.write_resource::<AudioSink>();
        sink.stop();
        *sink = AudioSink::new(&output);
        sink.set_volume(music_volume);

        let asset = data.world.read_resource::<AssetStorage<Source>>();
        let name =
            self.settings
                .song_names
                .get(self.level_index)
                .expect(&format!(
                    "Song name at index {} doesn't exist",
                    self.level_index
                ));
        let handle = data
            .world
            .write_resource::<AudioHandles>()
            .get_or_load(resource(format!("audio/{}", name)), &data.world);
        if let Some(sound) = asset.get(&handle) {
            sink.append(sound).unwrap();
        }
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
                self.save_to_savefile();
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

    fn save_to_savefile(&self) {
        let savefile_path = self.savefile_path();

        if let Some(player) = &self.player_checkpoint_opt {
            let savefile_data = savefile::SavefileData {
                player: player.clone(),
                levels: savefile::LevelsData {
                    current:   self.level_name(),
                    completed: self.completed_levels.clone(),
                },
            };

            match serde_json::to_string(&savefile_data) {
                Ok(serialized) => {
                    write_file(savefile_path, serialized).unwrap()
                }
                Err(err) => eprintln!(
                    "Couldn't save savefile data to file, an error occured \
                     while serializing save data:\n{:#?}",
                    err
                ),
            }
        }
    }

    fn load_from_file(&mut self) {
        let savefile_path = self.savefile_path();
        if let Ok(json_raw) = read_file(savefile_path) {
            match serde_json::from_str::<savefile::SavefileData>(&json_raw) {
                Ok(deserialized) => {
                    self.player_checkpoint_opt = Some(deserialized.player);
                    self.level_index =
                        self.level_index_from_name(deserialized.levels.current);
                    self.completed_levels = deserialized.levels.completed;
                }
                Err(err) => eprintln!(
                    "Couldn't load savefile data from file, an error occured \
                     while deserializing save data:\n{:#?}",
                    err
                ),
            }
        }
    }

    fn savefile_path(&self) -> String {
        use amethyst::utils::application_root_dir;
        format!("{}/{}", application_root_dir(), self.settings.savefile_path)
    }

    fn create_timer_ui<T>(
        &self,
        timer_type: TimerType,
        ui_settings: &crate::settings::SettingsTimerUi,
        ui_transform_name: T,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) where
        T: ToString,
    {
        let world = &mut data.world;

        let screen_size = data
            .data
            .custom
            .clone()
            .unwrap()
            .display_config
            .dimensions
            .unwrap_or((1200, 800));

        let font = world.read_resource::<Loader>().load(
            resource(&ui_settings.font_file),
            TtfFormat,
            Default::default(),
            (),
            &world.read_resource(),
        );

        let transform = new_ui_transform(
            &ui_transform_name.to_string(),
            AmethystAnchor::Middle,
            (0.0, 0.0, 0.0, screen_size.0 as f32, screen_size.1 as f32, 0),
        );

        let timer_ui = TimerUi {
            timer_type,
            text_prefix: ui_settings.text_prefix.clone(),
        };

        world
            .create_entity()
            .with(timer_ui)
            .with(transform)
            .with(UiText::new(
                font,
                String::new(),
                ui_settings.font_color,
                ui_settings.font_size,
            ))
            .build();
    }
}
