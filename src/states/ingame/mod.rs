mod level_loader;
mod level_manager;

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
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(&data.world, "ingame").unwrap();

        Trans::None
    }
}
