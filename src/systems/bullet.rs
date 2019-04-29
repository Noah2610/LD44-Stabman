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
        for (bullet_entity, bullet, bullet_collision) in
            (&entities, &mut bullets, &collisions).join()
        {
            // Collides with player?
            if bullet.owner != BulletOwner::Player {
                if let Some((player_entity, player, player_collision)) =
                    (&entities, &mut players, &collisions).join().next()
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
                        entities.delete(bullet_entity);
                    }
                }
            }
            // Collides with enemies?
            else if bullet.owner != BulletOwner::Enemy {
                for (enemy_entity, enemy, enemy_collision) in
                    (&entities, &mut enemies, &collisions).join()
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
                        entities.delete(bullet_entity);
                    }
                }
            }
            // Collides with solid?
            else {
                for (solid_entity, solid, enemy_collision) in
                    (&entities, &solids, &collisions).join()
                {
                    let solid_id = solid_entity.id();
                    if let Some(collision::Data {
                        state: collision::State::Enter,
                        ..
                    }) = bullet_collision.collision_with(solid_id)
                    {
                        entities.delete(bullet_entity);
                    }
                }
            }
        }
    }
}
