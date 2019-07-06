use super::system_prelude::*;

#[derive(Default)]
pub struct BulletCreatorSystem;

impl<'a> System<'a> for BulletCreatorSystem {
    type SystemData = (
        Write<'a, BulletCreator>,
        Entities<'a>,
        ReadExpect<'a, SpriteSheetHandles>,
        WriteStorage<'a, Bullet>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, Collision>,
        WriteStorage<'a, CheckCollision>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Transparent>,
        WriteStorage<'a, Flipped>,
    );

    fn run(
        &mut self,
        (
            mut bullet_creator,
            entities,
            spritesheet_handles,
            mut bullets,
            mut transforms,
            mut velocities,
            mut sizes,
            mut collisions,
            mut check_collisions,
            mut sprite_renders,
            mut animations,
            mut transparents,
            mut flippeds,
        ): Self::SystemData,
    ) {
        while let Some(BulletComponents {
            bullet,
            transform,
            velocity,
            size,
        }) = bullet_creator.pop()
        {
            let spritesheet_handle = spritesheet_handles
                .get("player_bullets")
                .expect("'player_bullets' spritesheet does not exist");
            let animation = Animation::new()
                .default_sprite_sheet_handle(spritesheet_handle.clone())
                .default_delay_ms(100)
                .sprite_ids(vec![0, 1, 2, 1])
                .build();
            let flipped = if velocity.x >= 0.0 {
                Flipped::None
            } else {
                Flipped::Horizontal
            };

            let entity = entities.create();
            bullets.insert(entity, bullet).unwrap();
            transforms.insert(entity, transform).unwrap();
            velocities.insert(entity, velocity).unwrap();
            sizes.insert(entity, size).unwrap();
            collisions.insert(entity, Collision::new()).unwrap();
            check_collisions.insert(entity, CheckCollision).unwrap();
            sprite_renders
                .insert(entity, SpriteRender {
                    sprite_sheet:  spritesheet_handle,
                    sprite_number: 0,
                })
                .unwrap();
            animations.insert(entity, animation).unwrap();
            transparents.insert(entity, Transparent).unwrap();
            flippeds.insert(entity, flipped).unwrap();
        }
    }
}
