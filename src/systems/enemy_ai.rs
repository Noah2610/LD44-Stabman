use std::time::Instant;

use deathframe::geo::Vector;

use super::system_prelude::*;

pub struct EnemyAiSystem;

impl<'a> System<'a> for EnemyAiSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        ReadStorage<'a, EnemyAi>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Solid>,
        ReadStorage<'a, Gravity>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Flipped>,
        WriteStorage<'a, DecreaseVelocity>,
        WriteStorage<'a, MaxVelocity>,
        WriteStorage<'a, AnimationsContainer>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            enemy_ais,
            transforms,
            collisions,
            solids,
            gravities,
            mut enemies,
            mut players,
            mut velocities,
            mut flippeds,
            mut decrease_velocities,
            mut max_velocities,
            mut animations_containers,
        ): Self::SystemData,
    ) {
        if let Some(player_data) = (&mut players, &transforms).join().find_map(
            |(player, transform)| {
                if player.in_control {
                    Some(PlayerData {
                        player,
                        pos: transform.into(),
                    })
                } else {
                    None
                }
            },
        ) {
            let dt = time.delta_seconds();
            let now = Instant::now();

            for (
                enemy_entity,
                enemy,
                enemy_ai,
                enemy_transform,
                enemy_velocity,
                enemy_flipped,
                enemy_decr_vel,
                enemy_max_vel,
                enemy_animations_container,
                enemy_collision,
                enemy_gravity_opt,
            ) in (
                &entities,
                &mut enemies,
                &enemy_ais,
                &transforms,
                &mut velocities,
                &mut flippeds,
                &mut decrease_velocities,
                &mut max_velocities,
                &mut animations_containers,
                &collisions,
                (&gravities).maybe(),
            )
                .join()
            {
                let sides_touching = SidesTouching::new(
                    &entities,
                    enemy_collision,
                    &collisions,
                    &solids,
                );

                // Run AI specific code
                match enemy_ai {
                    EnemyAi::Tracer => run_for_tracer_ai(
                        dt,
                        &player_data,
                        enemy,
                        enemy_transform,
                        enemy_velocity,
                    ),
                }

                // Reset y velocity if enemy has gravity and they are standing on a solid
                if (sides_touching.is_touching_bottom && enemy_velocity.y < 0.0)
                    || (sides_touching.is_touching_top
                        && enemy_velocity.y > 0.0)
                {
                    enemy_velocity.y = 0.0;
                }

                // Flip sprite
                if enemy_velocity.x > 0.0 {
                    if enemy_flipped == &mut Flipped::Horizontal {
                        *enemy_flipped = Flipped::None;
                    }
                    enemy_animations_container.set("walking");
                } else if enemy_velocity.x < 0.0 {
                    if enemy_flipped == &mut Flipped::None {
                        *enemy_flipped = Flipped::Horizontal;
                    }
                    enemy_animations_container.set("walking");
                } else {
                    enemy_animations_container.set("idle");
                }

                // Handle knockbacked state
                if let Some(knockbacked_at) = enemy.knockbacked_at {
                    if now.duration_since(knockbacked_at)
                        >= enemy.knockback_duration
                    {
                        enemy.knockbacked_at = None;
                    } else {
                        enemy_decr_vel.dont_decrease_x();
                        enemy_max_vel.dont_limit_x();
                    }
                }

                // Handle enemy death
                if enemy.is_dead() {
                    player_data.player.gain_reward(enemy.reward);
                    entities.delete(enemy_entity).unwrap();
                }
            }
        }
    }
}

fn run_for_tracer_ai<'a>(
    dt: f32,
    player_data: &PlayerData,
    enemy: &Enemy,
    transform: &Transform,
    velocity: &mut Velocity,
) {
    let enemy_pos = Vector::from(transform);
    let distance_to_player = (
        enemy_pos.0 - player_data.pos.0,
        enemy_pos.1 - player_data.pos.1,
    );

    if enemy.in_trigger_distance(distance_to_player.into()) {
        velocity.x +=
            enemy.acceleration.0 * -distance_to_player.0.signum() * dt;
        if enemy.acceleration.1 != 0.0 {
            velocity.y +=
                enemy.acceleration.1 * -distance_to_player.1.signum() * dt;
        }
    }
}

struct PlayerData<'a> {
    pub player: &'a mut Player,
    pub pos:    Vector,
}
