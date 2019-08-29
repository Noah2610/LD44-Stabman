use super::super::state_prelude::*;
use amethyst::assets::{Progress, ProgressCounter};
use amethyst::ecs::{Entities, Join, ReadStorage, Write};

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

    // fn created_ui_entity(
    //     &mut self,
    //     _data: &mut StateData<CustomGameData<CustomData>>,
    //     _entity: Entity,
    // ) {
    // }

    fn create_ui(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) -> ProgressCounter {
        let mut progress = ProgressCounter::new();

        let menu_entity = data.world.exec(|mut creator: UiCreator| {
            creator.create(resource(self.ui_ron_path()), &mut progress)
        });
        self.ui_entities_mut().push(menu_entity);

        progress

        // TODO
        // {
        //     let children = data.world.exec(
        //         |(entities, parents): (Entities, ReadStorage<Parent>)| {
        //             let mut all_children = Vec::new();
        //             let mut checked_parent_entities = Vec::new();
        //             let mut current_parent_entity = Some(menu_entity);
        //             while let Some(parent_entity) = current_parent_entity {
        //                 checked_parent_entities.push(parent_entity);

        //                 // Find children of current parent entity.
        //                 let mut children = (&entities, &parents)
        //                     .join()
        //                     .filter_map(|(entity, parent)| {
        //                         if parent.entity == parent_entity {
        //                             dbg!("FOUND CHILD");
        //                             Some(entity)
        //                         } else {
        //                             None
        //                         }
        //                     })
        //                     .collect();
        //                 all_children.append(&mut children);

        //                 // Find new parent entity to find children of, which hasn't been checked yet.
        //                 current_parent_entity = all_children
        //                     .iter()
        //                     .find(|entity| {
        //                         !checked_parent_entities.contains(entity)
        //                     })
        //                     .map(Clone::clone);
        //             }

        //             all_children
        //         },
        //     );

        //     for child in children {
        //         dbg!("CHILD");
        //         self.created_ui_entity(data, child);
        //     }
        // }
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
