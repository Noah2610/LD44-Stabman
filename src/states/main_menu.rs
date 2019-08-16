use super::state_prelude::*;
use amethyst::ecs::{Join, ReadStorage, Write};

#[derive(Default)]
pub struct MainMenu {
    ui_elements:  Vec<Entity>,
    ui_reader_id: Option<ReaderId<UiEvent>>,
}

impl MainMenu {
    fn create_ui(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        let main_menu_entity = data.world.exec(|mut creator: UiCreator| {
            creator.create(resource("ui/main_menu.ron"), ())
        });
        self.ui_elements.push(main_menu_entity);
    }

    fn delete_ui(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        data.world.delete_entities(&self.ui_elements).unwrap();
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
                                "start_button_normal" => {
                                    Some(Trans::Push(Box::new(Ingame::new(
                                        settings.clone(),
                                        CampaignType::Normal,
                                    ))))
                                }
                                "start_button_bonus" => {
                                    Some(Trans::Push(Box::new(Ingame::new(
                                        settings.clone(),
                                        CampaignType::Bonus,
                                    ))))
                                }
                                "quit_button" => Some(Trans::Quit),
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
        &self,
        data: &StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input_manager = data.world.input_manager();

        // Quit game
        if input_manager.is_up("decline") {
            Some(Trans::Quit)
        // Start game
        } else if input_manager.is_up("accept") {
            Some(Trans::Push(Box::new(Ingame::new(
                data.world.settings(),
                CampaignType::default(),
            ))))
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
        // Always set `ToMainMenu` resource to `false`
        data.world.write_resource::<ToMainMenu>().0 = false;
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
    }

    fn on_resume(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.create_ui(&mut data);
        // Always set `ToMainMenu` resource to `false`
        data.world.write_resource::<ToMainMenu>().0 = false;
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
