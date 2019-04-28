use deathframe::geo::Vector;
use deathframe::handlers::SpriteSheetHandles;

use super::system_prelude::*;

const FULL_HEART_SPRITE_ID: usize = 0;
const HALF_HEART_SPRITE_ID: usize = 1;
const HEART_SIZE: (f32, f32) = (8.0, 8.0);
const MARGIN: (f32, f32) = (8.0, 8.0);
const PADDING: f32 = 2.0;
const Z: f32 = 5.0;

#[derive(Default)]
pub struct HealthDisplaySystem {
    previous_health: u32,
}

impl<'a> System<'a> for HealthDisplaySystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, SpriteSheetHandles>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Camera>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, ScaleOnce>,
        WriteStorage<'a, Heart>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Transparent>,
    );

    fn run(
        &mut self,
        (
            entities,
            sprite_sheet_handles,
            players,
            cameras,
            mut transforms,
            mut sizes,
            mut scale_onces,
            mut hearts,
            mut sprite_renders,
            mut transparents,
        ): Self::SystemData,
    ) {
        let health_opt =
            (&players).join().find_map(|player| Some(player.health));
        let offset = (&cameras, &transforms)
            .join()
            .find_map(|(_, transform)| Some(transform.into()))
            .unwrap_or(Vector::new(0.0, 0.0));

        if let Some(health) = health_opt {
            if health == self.previous_health {
                for (transform, heart) in (&mut transforms, &hearts).join() {
                    let (x, y, z) = transform_xyz_for(heart.index, offset);
                    transform.set_xyz(x, y, z);
                }
            } else {
                self.previous_health = health;

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
                        &mut hearts,
                        &mut sprite_renders,
                        &mut transparents,
                        i,
                        offset,
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
                        &mut hearts,
                        &mut sprite_renders,
                        &mut transparents,
                        i,
                        offset,
                        HALF_HEART_SPRITE_ID,
                    );
                }
            }
        }
    }
}

fn create_heart<'a>(
    entities: &Entities<'a>,
    sprite_sheet_handles: &SpriteSheetHandles,
    transforms: &mut WriteStorage<'a, Transform>,
    sizes: &mut WriteStorage<'a, Size>,
    scale_onces: &mut WriteStorage<'a, ScaleOnce>,
    hearts: &mut WriteStorage<'a, Heart>,
    sprite_renders: &mut WriteStorage<'a, SpriteRender>,
    transparents: &mut WriteStorage<'a, Transparent>,
    i: u32,
    offset: Vector,
    sprite_id: usize,
) {
    let entity = entities.create();

    let mut transform = Transform::default();
    let (x, y, z) = transform_xyz_for(i, offset);
    transform.set_xyz(x, y, z);

    transforms.insert(entity, transform).unwrap();
    sizes.insert(entity, Size::from(HEART_SIZE)).unwrap();
    scale_onces.insert(entity, ScaleOnce).unwrap();
    hearts.insert(entity, Heart::new(i)).unwrap();
    sprite_renders
        .insert(entity, SpriteRender {
            sprite_sheet:  sprite_sheet_handles
                .get("player_hearts")
                .expect("Spritesheet 'player_hearts' does not exist"),
            sprite_number: sprite_id,
        })
        .unwrap();
    transparents.insert(entity, Transparent).unwrap();
}

fn transform_xyz_for(i: u32, offset: Vector) -> (f32, f32, f32) {
    (
        offset.0 + MARGIN.0 + (HEART_SIZE.0 + PADDING) * (i as f32),
        offset.1 + MARGIN.1,
        Z,
    )
}
