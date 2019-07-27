mod charger;
mod tracer;
mod turret;

use deathframe::geo::Vector;
use std::time::Instant;

use super::system_prelude::*;
use crate::settings::prelude::*;

struct PlayerData<'a> {
    pub player: &'a mut Player,
    pub pos:    Vector,
}

pub struct EnemyAiSystem;

impl<'a> System<'a> for EnemyAiSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Settings>,
        Read<'a, Time>,
        Write<'a, BulletCreator>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Solid<SolidTag>>,
        ReadStorage<'a, Gravity>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, EnemyAi>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Flipped>,
        WriteStorage<'a, DecreaseVelocity>,
        // WriteStorage<'a, MaxVelocity>,
        WriteStorage<'a, AnimationsContainer>,
    );

    fn run(
        &mut self,
        (
            entities,
            settings,
            time,
            mut bullet_creator,
            transforms,
            collisions,
            solids,
            gravities,
            mut enemies,
            mut enemy_ais,
            mut players,
            mut velocities,
            mut flippeds,
            mut decrease_velocities,
            // mut max_velocities,
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
                // enemy_max_vel,
                enemy_animations_container,
                enemy_collision,
                enemy_solid,
                enemy_gravity_opt,
            ) in (
                &entities,
                &mut enemies,
                &mut enemy_ais,
                &transforms,
                &mut velocities,
                &mut flippeds,
                &mut decrease_velocities,
                // &mut max_velocities,
                &mut animations_containers,
                &collisions,
                &solids,
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
                    EnemyAi::Tracer => tracer::run(
                        dt,
                        &player_data,
                        enemy,
                        enemy_transform,
                        enemy_velocity,
                        enemy_decr_vel,
                    ),
                    EnemyAi::Charger(data) => charger::run(
                        dt,
                        &player_data,
                        enemy,
                        data,
                        enemy_transform,
                        enemy_velocity,
                        enemy_decr_vel,
                        enemy_collision,
                        enemy_solid,
                        &entities,
                        &solids,
                    ),
                    EnemyAi::Turret(data) => turret::run(
                        dt,
                        &player_data,
                        enemy,
                        data,
                        enemy_transform,
                        &mut bullet_creator,
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
                    enemy_animations_container.set_if_has("walking");
                } else if enemy_velocity.x < 0.0 {
                    if enemy_flipped == &mut Flipped::None {
                        *enemy_flipped = Flipped::Horizontal;
                    }
                    enemy_animations_container.set_if_has("walking");
                } else {
                    enemy_animations_container.set_if_has("idle");
                }

                // Kill the enemies when they fall below the death_floor
                if enemy_transform.translation().y < settings.death_floor {
                    enemy.health = 0;
                }

                // Handle enemy death
                if enemy.is_dead() {
                    player_data.player.add_health(enemy.reward);
                    entities.delete(enemy_entity).unwrap();
                }
            }
        }
    }
}
