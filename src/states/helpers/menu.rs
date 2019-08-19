use super::super::state_prelude::*;
use amethyst::ecs::{Join, ReadStorage, Write};

pub trait Menu {
    /// Returns the path to the UI's RON configuration file.
    fn ui_ron_path(&self) -> &str;

    /// Returns a reference to the Vec of UI entities.
    fn ui_entities(&self) -> &Vec<Entity>;

    /// Returns a mutable reference to the Vec of UI entities.
    fn ui_entities_mut(&mut self) -> &mut Vec<Entity>;

    /// Returns a reference to an Option of ReaderId.
    fn ui_reader_id(&self) -> &Option<ReaderId<UiEvent>>;

    /// Returns a mutable reference to an Option of ReaderId.
    fn ui_reader_id_mut(&mut self) -> &mut Option<ReaderId<UiEvent>>;

    fn event_triggered<'a, 'b>(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
        event_name: String,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>>;

    fn create_ui(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        let menu_entity = data.world.exec(|mut creator: UiCreator| {
            creator.create(resource(self.ui_ron_path()), ())
        });
        self.ui_entities_mut().push(menu_entity);
    }

    fn delete_ui(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
        data.world.delete_entities(self.ui_entities()).unwrap();
        self.ui_entities_mut().clear();
    }

    fn update_ui_events<'a, 'b>(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let mut triggered_event = None;

        data.world.exec(
            |(entities, mut events, ui_transforms): (
                Entities,
                Write<EventChannel<UiEvent>>,
                ReadStorage<UiTransform>,
            )| {
                let reader_id = self
                    .ui_reader_id_mut()
                    .get_or_insert_with(|| events.register_reader());

                for event in events.read(reader_id) {
                    if let UiEventType::ClickStop = event.event_type {
                        let target_entity_id = event.target.id();
                        if let Some(name) = (&entities, &ui_transforms)
                            .join()
                            .find_map(|(entity, transform)| {
                                if entity.id() == target_entity_id {
                                    Some(transform.id.to_string())
                                } else {
                                    None
                                }
                            })
                        {
                            triggered_event = Some(name);
                        }
                    }
                }
            },
        );

        if let Some(event_name) = triggered_event {
            let trans_opt = self.event_triggered(data, event_name);
            if trans_opt.is_some() {
                trans_opt
            } else {
                None
            }
        } else {
            None
        }
    }
}
