use amethyst::ecs::{Join, ReadStorage, WriteStorage};

use super::super::state_prelude::*;
use super::level_loader::LevelLoader;

pub mod prelude {
    pub use super::LevelManager;
}

pub struct LevelManager {
    pub settings:          SettingsLevelManager,
    pub level_index:       usize,
    player_checkpoint_opt: Option<Player>,
}

impl LevelManager {
    pub fn new(settings: SettingsLevelManager) -> Self {
        Self {
            settings,
            level_index: 0,
            player_checkpoint_opt: None,
        }
    }

    pub fn load_current_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        data.world.delete_all(); // Remove _ALL_ existing entities first
        let current_level_name = self
            .settings
            .level_names
            .get(self.level_index)
            .expect(&format!(
                "Level at index {} does not exist",
                self.level_index
            ));
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
    }

    pub fn set_player_checkpoint(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        self.player_checkpoint_opt =
            data.world.exec(|players: ReadStorage<Player>| {
                (&players).join().next().map(Clone::clone)
            });
    }

    pub fn load_next_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        if self.has_next_level() {
            self.level_index += 1;
            self.load_current_level(data);
        } else {
            panic!("There is no next level");
        }
    }

    pub fn has_next_level(&self) -> bool {
        self.level_index + 1 < self.settings.level_names.len()
    }

    pub fn restart_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        self.load_current_level(data);
    }
}
