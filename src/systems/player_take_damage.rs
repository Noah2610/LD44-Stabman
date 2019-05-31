use super::system_prelude::*;

pub struct PlayerTakeDamageSystem;

impl<'a> System<'a> for PlayerTakeDamageSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, AnimationsContainer>,
    );

    fn run(
        &mut self,
        (
            entities,
            transforms,
            collisions,
            enemies,
            mut players,
            mut velocities,
            mut animations_containers,
        ): Self::SystemData,
    ) {
        for (
            player,
            player_transform,
            player_collision,
            player_velocity,
            player_animations_container,
        ) in (
            &mut players,
            &transforms,
            &collisions,
            &mut velocities,
            &mut animations_containers,
        )
            .join()
        {
            if player.in_control {
                for (enemy_entity, enemy, enemy_transform) in
                    (&entities, &enemies, &transforms).join()
                {
                    let enemy_id = enemy_entity.id();

                    if let Some(collision::Data {
                        state: collision::State::Enter,
                        ..
                    }) = player_collision.collision_with(enemy_id)
                    {
                        // Take damage
                        enemy.deal_damage_to(player);

                        // Knockback
                        // Figure out which direction to knock the player into by comparing the
                        // player's and the enemy's positions to each other.
                        let player_pos = player_transform.translation();
                        let enemy_pos = enemy_transform.translation();
                        let knockback = (
                            if player_pos.x > enemy_pos.x {
                                enemy.knockback.0
                            } else if player_pos.x < enemy_pos.x {
                                enemy.knockback.0 * -1.0
                            } else {
                                0.0
                            },
                            if player_pos.y > enemy_pos.y {
                                enemy.knockback.1
                            } else if player_pos.y < enemy_pos.y {
                                enemy.knockback.1 * -1.0
                            } else {
                                0.0
                            },
                        );
                        player_velocity.x = knockback.0;
                        player_velocity.y = knockback.1;
                    }
                }

                // Play death animation if killed
                if player.is_dead() {
                    player.in_control = false;
                    player_animations_container.play("death");
                }
            }
        }
    }
}
