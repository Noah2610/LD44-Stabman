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
                .default_delay_ms(50)
                .sprite_ids(vec![0, 1, 2])
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

// fn shoot_bullet<'a>(
//     entities: &Entities<'a>,
//     spritesheet_handles: &SpriteSheetHandles,
//     players: &WriteStorage<'a, Player>,
//     transforms: &mut WriteStorage<'a, Transform>,
//     velocities: &mut WriteStorage<'a, Velocity>,
//     sizes: &mut WriteStorage<'a, Size>,
//     flippeds: &mut WriteStorage<'a, Flipped>,
//     bullets: &mut WriteStorage<'a, Bullet>,
//     collisions: &mut WriteStorage<'a, Collision>,
//     check_collisions: &mut WriteStorage<'a, CheckCollision>,
//     sprite_renders: &mut WriteStorage<'a, SpriteRender>,
//     animations: &mut WriteStorage<'a, Animation>,
//     transparents: &mut WriteStorage<'a, Transparent>,
// ) {
//     let player_data_opt = (players, &*transforms, &*flippeds)
//         .join()
//         .next()
//         .map(|(player, transform, flipped)| {
//             let trans = transform.translation();
//             (player, (trans.x, trans.y, trans.z), flipped)
//         });

//     if let Some((player, player_pos, player_flipped)) = player_data_opt {
//         let spritesheet_handle = spritesheet_handles
//             .get("player_bullets")
//             .expect("'player_bullets' spritesheet does not exist");
//         let entity = entities.create();
//         bullets
//             .insert(
//                 entity,
//                 Bullet::new()
//                     .owner(BulletOwner::Player)
//                     .damage(player.items_data.bullet_damage)
//                     .lifetime(player.items_data.bullet_lifetime)
//                     .build(),
//             )
//             .unwrap();
//         collisions.insert(entity, Collision::new()).unwrap();
//         check_collisions.insert(entity, CheckCollision).unwrap();
//         let mut transform = Transform::default();
//         transform.set_xyz(player_pos.0, player_pos.1, player_pos.2);
//         transforms.insert(entity, transform).unwrap();
//         velocities
//             .insert(
//                 entity,
//                 Velocity::new(
//                     player.items_data.bullet_velocity.0
//                         * match player_flipped {
//                             Flipped::None => 1.0,
//                             Flipped::Horizontal => -1.0,
//                             _ => 1.0,
//                         },
//                     player.items_data.bullet_velocity.1,
//                 ),
//             )
//             .unwrap();
//         sizes
//             .insert(entity, Size::from(player.items_data.bullet_size))
//             .unwrap();
//         sprite_renders
//             .insert(entity, SpriteRender {
//                 sprite_sheet:  spritesheet_handle.clone(),
//                 sprite_number: 0,
//             })
//             .unwrap();
//         animations
//             .insert(
//                 entity,
//                 Animation::new()
//                     .default_sprite_sheet_handle(spritesheet_handle)
//                     .default_delay_ms(50)
//                     .sprite_ids(vec![0, 1, 2])
//                     .build(),
//             )
//             .unwrap();
//         transparents.insert(entity, Transparent).unwrap();
//         flippeds.insert(entity, player_flipped.clone()).unwrap();
//     }
// }
