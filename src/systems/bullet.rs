use std::time::Instant;

use super::system_prelude::*;

pub struct BulletSystem;

impl<'a> System<'a> for BulletSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Solid<SolidTag>>,
        WriteStorage<'a, Bullet>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, HeartsContainer>,
    );

    fn run(
        &mut self,
        (
            entities,
            collisions,
            solids,
            mut bullets,
            mut players,
            mut enemies,
            mut velocities,
            mut hearts_containers,
        ): Self::SystemData,
    ) {
        let now = Instant::now();

        for (bullet_entity, bullet, bullet_collision) in
            (&entities, &mut bullets, &collisions).join()
        {
            // Collides with player?
            if bullet.owner != BulletOwner::Player {
                if let Some((player_entity, player, player_velocity)) =
                    (&entities, &mut players, &mut velocities).join().next()
                {
                    let player_id = player_entity.id();
                    if let Some(collision::Data {
                        state: collision::State::Enter,
                        ..
                    }) = bullet_collision.collision_with(player_id)
                    {
                        // Bullet is colliding with player;
                        // deal damage to player and delete bullet entity.
                        player.take_damage(bullet.damage);
                        // Knockback
                        let knockback_opt =
                            match (&bullet.knockback, &bullet.facing) {
                                (Some(knockback), Some(facing)) => match facing
                                {
                                    Facing::Left => {
                                        Some((-knockback.0, knockback.1))
                                    }
                                    Facing::Right => {
                                        Some((knockback.0, knockback.1))
                                    }
                                },
                                (Some(knockback), None) => {
                                    None
                                    // (
                                    //     if player_pos.x > enemy_pos.x {
                                    //         enemy.knockback.0
                                    //     } else if player_pos.x < enemy_pos.x {
                                    //         enemy.knockback.0 * -1.0
                                    //     } else {
                                    //         0.0
                                    //     },
                                    //     if player_pos.y > enemy_pos.y {
                                    //         enemy.knockback.1
                                    //     } else if player_pos.y < enemy_pos.y {
                                    //         enemy.knockback.1 * -1.0
                                    //     } else {
                                    //         0.0
                                    //     },
                                    // );
                                }
                                _ => None,
                            };
                        if let Some(knockback) = knockback_opt {
                            player_velocity.x = knockback.0;
                            player_velocity.y = knockback.1;
                        }
                        entities.delete(bullet_entity).unwrap();
                    }
                }
            }
            // Collides with enemies?
            else if bullet.owner != BulletOwner::Enemy {
                for (enemy_entity, enemy, enemy_hearts_container_opt) in
                    (&entities, &mut enemies, (&mut hearts_containers).maybe())
                        .join()
                {
                    let enemy_id = enemy_entity.id();
                    if let Some(collision::Data {
                        state: collision::State::Enter,
                        ..
                    }) = bullet_collision.collision_with(enemy_id)
                    {
                        // Bullet is colliding with enemy;
                        // deal damage to enemy and delete bullet entity.
                        enemy.take_damage(bullet.damage);
                        entities.delete(bullet_entity).unwrap();
                        // Update HeartsContainer
                        if let Some(hearts_container) =
                            enemy_hearts_container_opt
                        {
                            hearts_container.health = enemy.health;
                        }
                    }
                }
            }

            // Collides with solid? (SolidTag::Default)
            for (solid_entity, solid, _) in
                (&entities, &solids, !&players).join()
            {
                let solid_id = solid_entity.id();
                if let SolidTag::Default = solid.tag {
                    if let Some(collision::Data {
                        state: collision::State::Enter,
                        ..
                    }) = bullet_collision.collision_with(solid_id)
                    {
                        entities.delete(bullet_entity).unwrap();
                    }
                }
            }

            // Delete bullet when its lifetime ends
            if now.duration_since(bullet.created_at) >= bullet.lifetime {
                entities.delete(bullet_entity).unwrap();
            }
        }
    }
}
