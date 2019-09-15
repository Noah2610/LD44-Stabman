use std::time::Instant;

use super::system_prelude::*;

pub struct BulletSystem;

impl<'a> System<'a> for BulletSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, World>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Solid<SolidTag>>,
        ReadStorage<'a, Invincible>,
        WriteStorage<'a, Bullet>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut world,
            collisions,
            solids,
            invincibles,
            mut bullets,
            mut players,
            mut enemies,
            mut velocities,
        ): Self::SystemData,
    ) {
        let now = Instant::now();
        let mut call_world_maintain = false;

        for (bullet_entity, bullet, bullet_collision) in
            (&entities, &mut bullets, &collisions).join()
        {
            // Collides with player?
            if bullet.owner != BulletOwner::Player {
                if let Some((player_entity, player, player_velocity, _)) =
                    (&entities, &mut players, &mut velocities, !&invincibles)
                        .join()
                        .next()
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
                                _ => None,
                            };
                        if let Some(knockback) = knockback_opt {
                            player_velocity.x = knockback.0;
                            player_velocity.y = knockback.1;
                        }
                        entities.delete(bullet_entity).unwrap();
                        call_world_maintain = true;
                    }
                }
            }
            // Collides with enemies?
            else if bullet.owner != BulletOwner::Enemy {
                for (enemy_entity, enemy, _) in
                    (&entities, &mut enemies, !&invincibles).join()
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
                        call_world_maintain = true;
                    }
                }
            }

            // Collides with solid? (SolidTag::Default)
            for (solid_entity, solid, _, _) in
                (&entities, &solids, !&players, !&enemies).join()
            {
                let solid_id = solid_entity.id();
                if let SolidTag::Default = solid.tag {
                    if let Some(collision::Data {
                        state: collision::State::Enter,
                        ..
                    }) = bullet_collision.collision_with(solid_id)
                    {
                        entities.delete(bullet_entity).unwrap();
                        call_world_maintain = true;
                    }
                }
            }

            // Delete bullet when its lifetime ends
            if now.duration_since(bullet.created_at) >= bullet.lifetime {
                entities.delete(bullet_entity).unwrap();
                call_world_maintain = true;
            }
        }

        if call_world_maintain {
            world.maintain();
        }
    }
}
