use super::super::state_prelude::*;
use super::level_loader::LevelLoader;

pub mod prelude {
    pub use super::LevelManager;
}

pub struct LevelManager {
    settings:    SettingsLevelManager,
    level_index: usize,
}

impl LevelManager {
    pub fn new(settings: SettingsLevelManager) -> Self {
        Self {
            settings,
            level_index: 0,
        }
    }

    pub fn load_current_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
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
    }
}
