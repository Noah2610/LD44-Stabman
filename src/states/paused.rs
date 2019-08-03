use super::state_prelude::*;
use amethyst::ecs::{Join, ReadStorage, Write};

#[derive(Default)]
pub struct Paused {
    ui_elements:  Vec<Entity>,
    ui_reader_id: Option<ReaderId<UiEvent>>,
    to_main_menu: bool,
}

impl Paused {
    fn create_ui(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        let pause_entity = data.world.exec(|mut creator: UiCreator| {
            creator.create(resource("ui/paused/pause_button.ron"), ())
        });
        self.ui_elements.push(pause_entity);
    }

    fn delete_ui(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        data.world.delete_entities(&self.ui_elements).unwrap();
        self.ui_elements.clear();
    }

    fn update_ui_events<'a, 'b>(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        data.world.exec(
            |(entities, mut events, ui_transforms): (
                Entities,
                Write<EventChannel<UiEvent>>,
                ReadStorage<UiTransform>,
            )| {
                let reader_id = self
                    .ui_reader_id
                    .get_or_insert_with(|| events.register_reader());

                for event in events.read(reader_id) {
                    if let UiEventType::ClickStop = event.event_type {
                        let target_entity_id = event.target.id();
                        if let Some(name) = (&entities, &ui_transforms)
                            .join()
                            .find_map(|(entity, transform)| {
                                if entity.id() == target_entity_id {
                                    Some(transform.id.as_ref())
                                } else {
                                    None
                                }
                            })
                        {
                            let trans_opt = match name {
                                "pause_button" => Some(Trans::Pop),
                                "quit_button" => {
                                    self.to_main_menu = true;
                                    Some(Trans::Pop)
                                }
                                _ => None,
                            };
                            if trans_opt.is_some() {
                                return trans_opt;
                            }
                        }
                    }
                }
                None
            },
        )
    }

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
