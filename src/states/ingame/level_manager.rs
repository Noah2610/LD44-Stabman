use super::super::state_prelude::*;

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
        // TODO
    }
}
