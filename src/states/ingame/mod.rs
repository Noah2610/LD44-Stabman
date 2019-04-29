mod level_loader;
mod level_manager;

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
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.level_manager.load_current_level(&mut data);
    }

    fn update(
        &mut self,
        mut data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(&data.world, "ingame").unwrap();

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
                    .unwrap_or(false)
                    && (&players, &animations_containers)
                        .join()
                        .find_map(|(_, animations_container)| {
                            Some(animations_container.play_once.is_none())
                        })
                        .unwrap_or(false);
                let player_dead = (&players, &animations_containers)
                    .join()
                    .find_map(|(player, animations_container)| {
                        Some(
                            player.is_dead()
                                && animations_container.play_once.is_none(),
                        )
                    })
                    .unwrap_or(false);
                (next_level, player_dead)
            },
        );
        if next_level {
            if self.level_manager.has_next_level() {
                self.level_manager.set_player_checkpoint(&mut data);
                self.level_manager.load_next_level(&mut data);
            } else {
                // TODO: Beat game!
                println!("You win!");
            }
        } else if player_dead {
            // Restart level and load player from checkoint
            self.level_manager.restart_level(&mut data);
        }

        Trans::None
    }
}
