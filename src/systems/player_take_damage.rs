use super::system_prelude::*;

pub struct PlayerTakeDamageSystem;

impl<'a> System<'a> for PlayerTakeDamageSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Flipped>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, AnimationsContainer>,
    );

    fn run(
        &mut self,
        (
            entities,
            collisions,
            enemies,
            flippeds,
            mut players,
            mut velocities,
            mut animations_containers,
        ): Self::SystemData,
    ) {
        for (
            player,
            player_collision,
            player_velocity,
            player_flipped,
            player_animations_container,
        ) in (
            &mut players,
            &collisions,
            &mut velocities,
            &flippeds,
            &mut animations_containers,
        )
            .join()
        {
            if player.in_control {
                for (enemy_entity, enemy) in (&entities, &enemies).join() {
                    let enemy_id = enemy_entity.id();

                    if let Some(collision::Data {
                        side,
                        state: collision::State::Enter,
                        ..
                    }) = player_collision.collision_with(enemy_id)
                    {
                        // Take damage
                        enemy.deal_damage_to(player);

                        // Knockback
                        let knockback = match side {
                            Side::Left => {
                                (enemy.knockback.0, enemy.knockback.1)
                            }
                            Side::Right => {
                                (enemy.knockback.0 * -1.0, enemy.knockback.1)
                            }
                            Side::Bottom | Side::Inner => (
                                x_knockback_for_vertical_side(
                                    enemy.knockback.0,
                                    player_flipped,
                                ),
                                enemy.knockback.1,
                            ),
                            Side::Top => (
                                x_knockback_for_vertical_side(
                                    enemy.knockback.0,
                                    player_flipped,
                                ),
                                enemy.knockback.1 * -1.0,
                            ),
                        };
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

fn x_knockback_for_vertical_side(knockback: f32, flipped: &Flipped) -> f32 {
    if let Flipped::None = flipped {
        knockback * -1.0
    } else {
        knockback
    }
}
