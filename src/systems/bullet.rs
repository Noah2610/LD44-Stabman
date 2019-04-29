use std::time::Instant;

use super::system_prelude::*;

pub struct BulletSystem;

impl<'a> System<'a> for BulletSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Solid>,
        WriteStorage<'a, Bullet>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Enemy>,
    );

    fn run(
        &mut self,
        (entities, collisions, solids, mut bullets, mut players, mut enemies): Self::SystemData,
    ) {
        let now = Instant::now();

        for (bullet_entity, bullet, bullet_collision) in
            (&entities, &mut bullets, &collisions).join()
        {
            // Collides with player?
            if bullet.owner != BulletOwner::Player {
                if let Some((player_entity, player)) =
                    (&entities, &mut players).join().next()
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
                        entities.delete(bullet_entity).unwrap();
                    }
                }
            }
            // Collides with enemies?
            else if bullet.owner != BulletOwner::Enemy {
                for (enemy_entity, enemy) in (&entities, &mut enemies).join() {
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
                    }
                }
            }

            // Collides with solid?
            for (solid_entity, _, _) in (&entities, &solids, !&players).join() {
                let solid_id = solid_entity.id();
                if let Some(collision::Data {
                    state: collision::State::Enter,
                    ..
                }) = bullet_collision.collision_with(solid_id)
                {
                    entities.delete(bullet_entity).unwrap();
                }
            }

            // Delete bullet when its lifetime ends
            if now.duration_since(bullet.created_at) >= bullet.lifetime {
                entities.delete(bullet_entity).unwrap();
            }
        }
    }
}
