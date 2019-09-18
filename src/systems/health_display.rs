use deathframe::geo::Vector;
use deathframe::handlers::SpriteSheetHandles;

use super::system_prelude::*;
use crate::resources::UpdateHealthDisplay;

// TODO: Put these values into settings.ron
const FULL_HEART_SPRITE_ID: usize = 0;
const HALF_HEART_SPRITE_ID: usize = 1;
const HEART_SIZE: (f32, f32) = (16.0, 16.0);
const MARGIN: (f32, f32) = (16.0, 14.0);
const PADDING: f32 = 4.0;
const Z: f32 = 5.0;

#[derive(Default)]
pub struct HealthDisplaySystem {
    previous_health: u32,
}

impl<'a> System<'a> for HealthDisplaySystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, SpriteSheetHandles>,
        Write<'a, UpdateHealthDisplay>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Camera>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, ScaleOnce>,
        WriteStorage<'a, PlayerHeart>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transparent>,
        WriteStorage<'a, DontDeleteOnNextLevel>,
    );

    fn run(
        &mut self,
        (
            entities,
            sprite_sheet_handles,
            mut update_health_display,
            players,
            cameras,
            mut transforms,
            mut sizes,
            mut scale_onces,
            mut hearts,
            mut sprite_renders,
            mut transparents,
            mut dont_deletes,
        ): Self::SystemData,
    ) {
        let health_opt =
            (&players).join().find_map(|player| Some(player.health));
        let offset = (&cameras, &transforms)
            .join()
            .find_map(|(_, transform)| Some(transform.into()))
            .unwrap_or(Vector::new(0.0, 0.0));

        if let Some(health) = health_opt {
            if health == self.previous_health && !update_health_display.0 {
                for (transform, heart) in (&mut transforms, &hearts).join() {
                    let (x, y, z) = transform_xyz_for(heart.0.index, offset);
                    transform.set_xyz(x, y, z);
                }
            } else {
                self.previous_health = health;
                if update_health_display.0 {
                    update_health_display.0 = false;
                }

                // Delete all previous hearts
                for (entity, _) in (&entities, &hearts).join() {
                    entities.delete(entity).unwrap();
                }

                let full_hearts = health / 2;
                let half_hearts = health - full_hearts * 2;

                // Generate new full hearts
                for i in 0 .. full_hearts {
                    create_heart(
                        &entities,
                        &sprite_sheet_handles,
                        &mut transforms,
                        &mut sizes,
                        &mut scale_onces,
                        AnyHeart::Player(&mut hearts),
                        &mut sprite_renders,
                        &mut transparents,
                        Some(&mut dont_deletes),
                        i,
                        transform_xyz_for(i, offset),
                        HEART_SIZE.into(),
                        FULL_HEART_SPRITE_ID,
                    );
                }

                // Generate new half hearts
                for i in full_hearts .. full_hearts + half_hearts {
                    create_heart(
                        &entities,
                        &sprite_sheet_handles,
                        &mut transforms,
                        &mut sizes,
                        &mut scale_onces,
                        AnyHeart::Player(&mut hearts),
                        &mut sprite_renders,
                        &mut transparents,
                        Some(&mut dont_deletes),
                        i,
                        transform_xyz_for(i, offset),
                        HEART_SIZE.into(),
                        HALF_HEART_SPRITE_ID,
                    );
                }
            }
        }
    }
}

fn transform_xyz_for(i: u32, offset: Vector) -> (f32, f32, f32) {
    (
        offset.0 + MARGIN.0 + (HEART_SIZE.0 + PADDING) * (i as f32),
        offset.1 + MARGIN.1,
        Z,
    )
}
