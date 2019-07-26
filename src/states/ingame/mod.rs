mod level_loader;
mod level_manager;

use amethyst::audio::{output::Output, AudioSink, Source};
use amethyst::ecs::{Join, ReadStorage};

use super::state_prelude::*;
use level_manager::prelude::*;

pub struct Ingame {
    level_manager: LevelManager,
}

impl Ingame {
    pub fn new(settings: Settings) -> Self {
        Self {
            level_manager: LevelManager::new(settings.level_manager),
        }
    }

    fn play_current_song(
        &self,
        mut data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        let output = data.world.read_resource::<Output>();
        let mut sink = data.world.write_resource::<AudioSink>();
        sink.stop();
        *sink = AudioSink::new(&output);
        let asset = data.world.read_resource::<AssetStorage<Source>>();
        let name = self
            .level_manager
            .settings
            .song_names
            .get(self.level_manager.level_index)
            .expect(&format!(
                "Song name at index {} doesn't exist",
                self.level_manager.level_index
            ));
        let handle = data
            .world
            .write_resource::<AudioHandles>()
            .get_or_load(resource(format!("audio/{}", name)), &data.world);
        if let Some(sound) = asset.get(&handle) {
            sink.append(sound).unwrap();
        }
    }

    fn handle_keys<'a, 'b>(
        &self,
        data: &StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input_manager = data.world.input_manager();

        if input_manager.is_up("quit") {
            Some(Trans::Quit)
        } else {
            None
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.level_manager.load_current_level(&mut data);
    }

    fn handle_event(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) {
                Trans::Quit
            } else {
                Trans::None
            }
        } else {
            Trans::None
        }
    }

    fn update(
        &mut self,
        mut data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(&data.world, "ingame").unwrap();

        if let Some(trans) = self.handle_keys(&data) {
            return trans;
        }

        // Check if the level was beaten
        let (next_level, player_dead) = data.world.exec(
            |(goals, players, animations_containers): (
                ReadStorage<Goal>,
                ReadStorage<Player>,
                ReadStorage<AnimationsContainer>,
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
                let player_dead = (&players, &animations_containers)
                    .join()
                    .find_map(|(player, animations_container)| {
                        Some(
                            player.is_dead(), // && animations_container.play_once.is_none(),
                        )
                    })
                    .unwrap_or(false);
                (next_level, player_dead)
            },
        );
        if next_level {
            if self.level_manager.has_next_level() {
                self.level_manager.set_player_checkpoint(&mut data);
                self.level_manager.load_next_level(&mut data, true);
                self.play_current_song(&mut data);
            } else {
                // TODO: Beat game!
                println!("You win!");
            }
        } else if player_dead {
            // Restart level and load player from checkoint
            self.level_manager.restart_level(&mut data);
        } else {
            if data.world.read_resource::<AudioSink>().empty() {
                self.play_current_song(&mut data);
            }
        }

        Trans::None
    }
}
