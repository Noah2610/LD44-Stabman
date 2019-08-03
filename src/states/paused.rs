use super::state_prelude::*;
use amethyst::ecs::Write;

enum UiType {
    PauseButton,
}

#[derive(Default)]
pub struct Paused {
    ui_elements:  Vec<UiElement<UiType>>,
    ui_reader_id: Option<ReaderId<UiEvent>>,
    to_main_menu: bool,
}

impl Paused {
    fn create_ui(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        let pause_entity = data.world.exec(|mut creator: UiCreator| {
            creator.create(resource("ui/paused/pause_button.ron"), ())
        });
        self.ui_elements.push(UiElement {
            entity:  pause_entity,
            ui_type: UiType::PauseButton,
        });
    }

    fn delete_ui(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        data.world
            .delete_entities(
                &self
                    .ui_elements
                    .iter()
                    .map(|el| el.entity)
                    .collect::<Vec<Entity>>(),
            )
            .unwrap();
        self.ui_elements.clear();
    }

    fn update_ui_events<'a, 'b>(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        data.world.exec(|mut events: Write<EventChannel<UiEvent>>| {
            let reader_id = self
                .ui_reader_id
                .get_or_insert_with(|| events.register_reader());

            for event in events
                .read(reader_id)
                .filter(|e| e.event_type == UiEventType::ClickStop)
            {
                for UiElement { entity, ui_type } in self.ui_elements.iter() {
                    if let UiType::PauseButton = ui_type {
                        if entity.id() == event.target.id() {
                            // Clicked pause button
                            return Some(Trans::Pop);
                        }
                    }
                }
            }
            None
        })
    }

    fn handle_keys<'a, 'b>(
        &mut self,
        data: &StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        enum Action {
            Quit,
            Pause,
        }

        if let Some(action) = {
            let input_manager = data.world.input_manager();

            if input_manager.is_up("quit") {
                Some(Action::Quit)
            } else if input_manager.is_down("pause") {
                Some(Action::Pause)
            } else {
                None
            }
        } {
            match action {
                Action::Quit => {
                    // Return to main menu; it should tell the `Ingame` state, that it
                    // should immediately pop off as well.
                    self.to_main_menu = true;
                    Some(Trans::Pop)
                }
                Action::Pause => {
                    // Unpause
                    Some(Trans::Pop)
                }
            }
        } else {
            None
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Paused {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.create_ui(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
        if self.to_main_menu {
            // Create a resource to tell the `Ingame` state that
            // it should immediately pop off as well.
            data.world.write_resource::<ToMainMenu>().0 = true;
        }
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
