use super::state_prelude::*;
use amethyst::ecs::{Join, WriteStorage};

const UI_RON_PATH: &str = "ui/paused.ron";

#[derive(Default)]
pub struct Paused {
    ui_entities:  Vec<Entity>,
    ui_reader_id: Option<ReaderId<UiEvent>>,
    to_main_menu: bool,
}

impl Paused {
    fn handle_keys<'a, 'b>(
        &mut self,
        data: &StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input_manager = data.world.input_manager();

        // Return to main menu; it should tell the `Ingame` state, that it
        // should immediately pop off as well.
        if input_manager.is_up("decline") {
            self.to_main_menu = true;
            Some(Trans::Pop)
        // Unpause / Resume game
        } else if input_manager.is_down("pause")
            || input_manager.is_up("accept")
        {
            Some(Trans::Pop)
        } else {
            None
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Paused {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.create_ui(&mut data);

        // Pause timers
        {
            let mut timers = data.world.write_resource::<Timers>();
            if timers.level.state.is_running() {
                timers.level.pause().unwrap();
            }
            timers.global.as_mut().map(|timer| {
                if timer.state.is_running() {
                    timer.pause().unwrap()
                }
            });
        }

        // Pause all turret timers
        data.world.exec(|mut enemy_ais: WriteStorage<EnemyAi>| {
            (&mut enemy_ais).join().for_each(|enemy_ai| {
                if let EnemyAi::Turret(ai_data) = enemy_ai {
                    ai_data.shot_timer.pause().unwrap();
                }
            });
        });
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
        if self.to_main_menu {
            // Create a resource to tell the `Ingame` state that
            // it should immediately pop off as well.
            data.world.write_resource::<ToMainMenu>().0 = true;
        }

        // Resume timers
        {
            let mut timers = data.world.write_resource::<Timers>();
            if timers.level.state.is_paused() {
                timers.level.resume().unwrap();
            }
            timers.global.as_mut().map(|timer| {
                if timer.state.is_paused() {
                    timer.resume().unwrap()
                }
            });
        }

        // Resume all turret timers
        data.world.exec(|mut enemy_ais: WriteStorage<EnemyAi>| {
            (&mut enemy_ais).join().for_each(|enemy_ai| {
                if let EnemyAi::Turret(ai_data) = enemy_ai {
                    ai_data.shot_timer.resume().unwrap();
                }
            });
        });
    }

    fn handle_event(
        &mut self,
        _data: StateData<CustomGameData<CustomData>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(&data.world, "paused").unwrap();

        if let Some(trans) = self.handle_keys(&data) {
            return trans;
        }

        Trans::None
    }

    fn fixed_update(
        &mut self,
        mut data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        if let Some(trans) = self.update_ui_events(&mut data) {
            return trans;
        }
        Trans::None
    }
}

impl Menu for Paused {
    fn event_triggered<'a, 'b>(
        &mut self,
        _data: &mut StateData<CustomGameData<CustomData>>,
        event_name: String,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        match event_name.as_ref() {
            "pause_button" => Some(Trans::Pop),
            "quit_button" => {
                self.to_main_menu = true;
                Some(Trans::Pop)
            }
            _ => None,
        }
    }

    fn ui_ron_path(&self) -> &str {
        UI_RON_PATH
    }

    fn ui_entities(&self) -> &Vec<Entity> {
        &self.ui_entities
    }

    fn ui_entities_mut(&mut self) -> &mut Vec<Entity> {
        &mut self.ui_entities
    }

    fn ui_reader_id(&self) -> &Option<ReaderId<UiEvent>> {
        &self.ui_reader_id
    }

    fn ui_reader_id_mut(&mut self) -> &mut Option<ReaderId<UiEvent>> {
        &mut self.ui_reader_id
    }
}
