mod level_loader;
mod level_manager;

use super::state_prelude::*;
use climer::timer::Timer;
use level_manager::prelude::*;

pub struct Ingame {
    level_manager: LevelManager,
    to_main_menu:  bool,
}

impl Ingame {
    pub fn new(settings: Settings) -> Self {
        Self {
            level_manager: LevelManager::new(settings.level_manager),
            to_main_menu:  false,
        }
    }

    fn handle_keys<'a, 'b>(
        &self,
        data: &StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input_manager = data.world.input_manager();

        // Return to main menu
        if input_manager.is_up("decline") {
            Some(Trans::Pop)
        } else if input_manager.is_down("pause") {
            let paused_state = Box::new(Paused::default());
            Some(Trans::Push(paused_state))
        } else {
            None
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.level_manager.load_current_level(&mut data);
        // Force update `HealthDisplay`
        data.world.write_resource::<UpdateHealthDisplay>().0 = true;

        // Start global timer
        let mut timers = data.world.write_resource::<Timers>();
        timers.global.start().unwrap();
    }

    fn on_stop(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // Delete _ALL_ entities before
        data.world.delete_all();

        // Stop global timer
        let mut timers = data.world.write_resource::<Timers>();
        timers.global.stop().unwrap();
    }

    fn on_resume(&mut self, data: StateData<CustomGameData<CustomData>>) {
        // Return to main menu, if `Paused` state set the resource to do so
        self.to_main_menu = data.world.read_resource::<ToMainMenu>().0;
    }

    fn handle_event(
        &mut self,
        _data: StateData<CustomGameData<CustomData>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        mut data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        // Return to main menu, if necessary
        if self.to_main_menu {
            return Trans::Pop;
        }

        data.data.update(&data.world, "ingame").unwrap();

        if let Some(trans) = self.handle_keys(&data) {
            return trans;
        }

        self.level_manager.update(&mut data);

        Trans::None
    }
}
