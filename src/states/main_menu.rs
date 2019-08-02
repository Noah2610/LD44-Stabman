use super::state_prelude::*;
use amethyst::ecs::{Join, ReadStorage, Write};

enum UiType {
    MainMenu,
}

#[derive(Default)]
pub struct MainMenu {
    ui_elements:  Vec<UiElement<UiType>>,
    ui_reader_id: Option<ReaderId<UiEvent>>,
}

impl MainMenu {
    fn create_ui(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        let main_menu_entity = data.world.exec(|mut creator: UiCreator| {
            creator.create(resource("ui/main_menu/main_menu.ron"), ())
        });
        self.ui_elements.push(UiElement {
            entity:  main_menu_entity,
            ui_type: UiType::MainMenu,
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
        let settings = data.world.settings();

        data.world.exec(
            |(entities, mut events, ui_transforms): (
                Entities,
                Write<EventChannel<UiEvent>>,
                ReadStorage<UiTransform>,
            )| {
                let reader_id = self
                    .ui_reader_id
                    .get_or_insert_with(|| events.register_reader());

                for event in events
                    .read(reader_id)
                    .filter(|e| e.event_type == UiEventType::ClickStop)
                {
                    for UiElement { entity, ui_type } in self.ui_elements.iter()
                    {
                        if let UiType::MainMenu = ui_type {
                            if let Some(name) = (&entities, &ui_transforms)
                                .join()
                                .find(|(entity, transform)| {
                                    // TODO: This is stupid... (probably?)
                                    transform.id == "start_button"
                                })
                            {
                                // Clicked start button - push ingame state
                                return Some(Trans::Push(Box::new(
                                    Ingame::new(settings),
                                )));
                            }
                        }
                    }
                }
                None
            },
        )
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

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent>
    for MainMenu
{
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.create_ui(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
    }

    fn on_resume(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.create_ui(&mut data);
    }

    fn on_pause(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
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
        data.data.update(&data.world, "main_menu").unwrap();

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
