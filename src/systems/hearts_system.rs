use std::collections::HashMap;

use super::system_prelude::*;

// TODO
const FULL_HEART_SPRITE_ID: usize = 0;
const HALF_HEART_SPRITE_ID: usize = 1;
const Z: f32 = 5.0;

struct HeartsContainerData {
    pub hp:  u32,
    pub pos: Vector,
}

#[derive(Default)]
pub struct HeartsSystem {
    hearts_containers_data: HashMap<Index, HeartsContainerData>,
}

impl<'a> System<'a> for HeartsSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, SpriteSheetHandles>,
        WriteStorage<'a, HeartsContainer>,
        WriteStorage<'a, Heart>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transparent>,
        WriteStorage<'a, ScaleOnce>,
    );

    fn run(
        &mut self,
        (
            entities,
            sprite_sheet_handles,
            mut hearts_containers,
            mut hearts,
            mut transforms,
            mut sizes,
            mut sprite_renders,
            mut transparents,
            mut scale_onces,
        ): Self::SystemData,
    ) {
        let mut hearts_containers_to_update = Vec::new();

        // Figure out which hearts_containers need updating
        for (
            hearts_container_entity,
            hearts_container,
            hearts_container_transform,
        ) in (&entities, &hearts_containers, &transforms).join()
        {
            let hearts_container_id = hearts_container_entity.id();
            let hearts_container_pos = Vector::from(hearts_container_transform);

            if let Some(hearts_container_data) =
                self.hearts_containers_data.get(&hearts_container_id)
            {
                let hp_changed =
                    hearts_container.hp != hearts_container_data.hp;
                let pos_changed =
                    hearts_container_pos != hearts_container_data.pos;

                if hp_changed || pos_changed {
                    hearts_containers_to_update.push(
                        HeartsContainerUpdateData {
                            id:            hearts_container_id,
                            pos:           hearts_container_pos,
                            hp:            hearts_container.hp,
                            heart_ids:     hearts_container.heart_ids.clone(),
                            heart_size:    hearts_container.heart_size,
                            heart_padding: hearts_container.heart_padding,
                            hearts_action: if hp_changed {
                                HeartsUpdateAction::Recreate
                            } else {
                                HeartsUpdateAction::MoveTransforms
                            },
                        },
                    );
                }
            } else {
                hearts_containers_to_update.push(HeartsContainerUpdateData {
                    id:            hearts_container_id,
                    pos:           hearts_container_pos,
                    hp:            hearts_container.hp,
                    heart_ids:     hearts_container.heart_ids.clone(),
                    heart_size:    hearts_container.heart_size,
                    heart_padding: hearts_container.heart_padding,
                    hearts_action: HeartsUpdateAction::Recreate,
                });
            }
        }

        // let mut heart_ids_to_remove: Vec<Index> = Vec::new();
        let mut new_heart_ids_for_hearts_containers: HashMap<
            Index,
            Vec<Index>,
        > = HashMap::new();

        // Update necessary hearts_containers
        hearts_containers_to_update
            .iter_mut()
            .for_each(|update_data| {
                let amount_of_hearts = update_data.hp / 2 + update_data.hp % 2;
                let amount_of_hearts_halfed = amount_of_hearts as f32 * 0.5;

                let hearts_area = Rect {
                    top:    update_data.pos.1
                        + amount_of_hearts_halfed * update_data.heart_padding.1,
                    bottom: update_data.pos.1
                        - amount_of_hearts_halfed * update_data.heart_padding.1,
                    left:   update_data.pos.0
                        - amount_of_hearts_halfed
                            * (update_data.heart_size.0
                                + update_data.heart_padding.0),
                    right:  update_data.pos.0
                        + amount_of_hearts_halfed
                            * (update_data.heart_size.0
                                + update_data.heart_padding.0),
                };

                let len_axis_x = hearts_area.right - hearts_area.left;
                // let len_axis_y = hearts_area.top - hearts_area.bottom;

                match update_data.hearts_action {
                    HeartsUpdateAction::MoveTransforms => {
                        for (heart_entity, heart, heart_transform) in
                            (&entities, &hearts, &mut transforms).join()
                        {
                            let heart_id = heart_entity.id();
                            if update_data.heart_ids.contains(&heart_id) {
                                heart_transform.set_x(
                                    hearts_area.left
                                        + len_axis_x
                                            / (heart.index as f32 + 1.0),
                                );
                                heart_transform.set_y(hearts_area.top); // TODO
                            }
                        }
                    }

                    HeartsUpdateAction::Recreate => {
                        // Delete existing heart entities
                        for heart_id in update_data.heart_ids.iter() {
                            entities
                                .delete(entities.entity(*heart_id))
                                .unwrap();
                        }

                        // Create new heart entities
                        let full_hearts = update_data.hp / 2;
                        let half_hearts = update_data.hp - full_hearts * 2;

                        for i in 0 .. full_hearts {
                            let pos = (
                                hearts_area.left
                                    + len_axis_x / (i as f32 + 1.0),
                                hearts_area.top, // TODO
                                Z,               // TODO
                            );
                            create_heart(
                                &entities,
                                &sprite_sheet_handles,
                                &mut transforms,
                                &mut sizes,
                                &mut scale_onces,
                                AnyHeart::Normal(&mut hearts),
                                &mut sprite_renders,
                                &mut transparents,
                                None,
                                i,
                                pos,
                                update_data.heart_size,
                                FULL_HEART_SPRITE_ID,
                            );
                        }
                    }
                }
            });
    }
}

struct HeartsContainerUpdateData {
    pub id:            Index,
    pub pos:           Vector,
    pub hp:            u32,
    pub heart_ids:     Vec<Index>,
    pub heart_size:    Vector,
    pub heart_padding: Vector,
    pub hearts_action: HeartsUpdateAction,
}

enum HeartsUpdateAction {
    MoveTransforms,
    Recreate,
}
