use std::time::{Duration, Instant};

use deathframe::components::solid::SolidTag as _;
use deathframe::geo::Vector;
use deathframe::handlers::SpriteSheetHandles;

use super::system_prelude::*;
use crate::settings::prelude::*;

pub struct EnemyAiSystem;

impl<'a> System<'a> for EnemyAiSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Settings>,
        Read<'a, Time>,
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
                    EnemyAi::Tracer => run_for_tracer_ai(
                        dt,
                        &player_data,
                        enemy,
                        enemy_transform,
                        enemy_velocity,
                        enemy_decr_vel,
                    ),
                    EnemyAi::Charger(data) => run_for_charger_ai(
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
                    EnemyAi::Turret(data) => run_for_turret_ai(
                        dt,
                        &player_data,
                        enemy,
                        data,
                        enemy_transform,
                    ),
                    // dt: f32,
                    // player_data: &PlayerData,
                    // enemy: &Enemy,
                    // ai_data: &mut EnemyAiTurretData,
                    // transform: &Transform,
                    // flipped: &Flipped,
                    // entities: &Entities,
                    // spritesheet_handles: &SpriteSheetHandles,
                    // transforms: &mut WriteStorage<Transform>,
                    // velocities: &mut WriteStorage<Velocity>,
                    // sizes: &mut WriteStorage<Size>,
                    // flippeds: &mut WriteStorage<Flipped>,
                    // bullets: &mut WriteStorage<Bullet>,
                    // collisions: &mut WriteStorage<Collision>,
                    // check_collisions: &mut WriteStorage<CheckCollision>,
                    // sprite_renders: &mut WriteStorage<SpriteRender>,
                    // animations: &mut WriteStorage<Animation>,
                    // transparents: &mut WriteStorage<Transparent>,
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

                // TODO Cleanup
                // Handle knockbacked state
                // if let Some(knockbacked_at) = enemy.knockbacked_at {
                //     if now.duration_since(knockbacked_at)
                //         >= enemy.knockback_duration
                //     {
                //         enemy.knockbacked_at = None;
                //     } else {
                //         enemy_decr_vel.dont_decrease_x();
                //         enemy_max_vel.dont_limit_x();
                //     }
                // }

                // Kill the enemies when they fall below the death_floor
                if enemy_transform.translation().y < settings.death_floor {
                    enemy.health = 0;
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

/// Simply move towards the player, when they are within trigger distance.
fn run_for_tracer_ai<'a>(
    dt: f32,
    player_data: &PlayerData,
    enemy: &Enemy,
    transform: &Transform,
    velocity: &mut Velocity,
    decr_velocity: &mut DecreaseVelocity,
) {
    let enemy_pos = Vector::from(transform);
    let distance_to_player = (
        enemy_pos.0 - player_data.pos.0,
        enemy_pos.1 - player_data.pos.1,
    );

    if enemy.in_trigger_distance(distance_to_player) {
        let increase = Vector::new(
            if enemy.is_outside_deadzone_x(distance_to_player.0) {
                enemy.acceleration.0 * -distance_to_player.0.signum() * dt
            } else {
                0.0
            },
            if enemy.is_outside_deadzone_y(distance_to_player.1) {
                enemy.acceleration.1 * -distance_to_player.1.signum() * dt
            } else {
                0.0
            },
        );
        // Don't decrease velocity when moving
        if increase.0 > 0.0 {
            decr_velocity.dont_decrease_x_when_pos();
        } else if increase.0 < 0.0 {
            decr_velocity.dont_decrease_x_when_neg();
        }
        if increase.1 > 0.0 {
            decr_velocity.dont_decrease_y_when_pos();
        } else if increase.1 < 0.0 {
            decr_velocity.dont_decrease_y_when_neg();
        }
        // Increase velocity
        velocity.increase_with_max(increase, enemy.max_velocity);
    }
}

/// When the enemy sees the player, start moving in their direction;
/// don't stop moving until the enemy hits a wall.
fn run_for_charger_ai(
    dt: f32,
    player_data: &PlayerData,
    enemy: &Enemy,
    ai_data: &mut EnemyAiChargerData,
    transform: &Transform,
    velocity: &mut Velocity,
    decr_velocity: &mut DecreaseVelocity,
    collision: &Collision,
    solid: &Solid<SolidTag>,
    entities: &Entities,
    solids: &ReadStorage<Solid<SolidTag>>,
) {
    if ai_data.is_moving {
        // Increase velocity
        velocity.increase_with_max(ai_data.velocity, enemy.max_velocity);
        // Don't decrease velocity when moving
        if ai_data.velocity.0 > 0.0 {
            decr_velocity.dont_decrease_x_when_pos();
        } else if ai_data.velocity.0 < 0.0 {
            decr_velocity.dont_decrease_x_when_neg();
        }
        if ai_data.velocity.1 > 0.0 {
            decr_velocity.dont_decrease_y_when_pos();
        } else if ai_data.velocity.1 < 0.0 {
            decr_velocity.dont_decrease_y_when_neg();
        }
        // Check if in collision with solid
        let in_collision = if let Some(stop_moving_sides) =
            &ai_data.stop_moving_when_colliding_sides
        {
            (entities, solids).join().any(|(entity, other_solid)| {
                solid.tag.collides_with(&other_solid.tag)
                    && if let Some(coll_data) =
                        collision.collision_with(entity.id())
                    {
                        stop_moving_sides.contains(&coll_data.side)
                    } else {
                        false
                    }
            })
        } else {
            false
        };
        if in_collision {
            // Stop moving
            velocity.clear();
            ai_data.is_moving = false;
        }
    } else {
        let enemy_pos = Vector::from(transform);
        let distance_to_player = (
            enemy_pos.0 - player_data.pos.0,
            enemy_pos.1 - player_data.pos.1,
        );
        if enemy.in_trigger_distance(distance_to_player) {
            // Start moving
            let increase = Vector::new(
                if enemy.is_outside_deadzone_x(distance_to_player.0) {
                    enemy.acceleration.0 * -distance_to_player.0.signum() * dt
                } else {
                    0.0
                },
                if enemy.is_outside_deadzone_y(distance_to_player.1) {
                    enemy.acceleration.1 * -distance_to_player.1.signum() * dt
                } else {
                    0.0
                },
            );
            // Don't decrease velocity when moving
            if increase.0 > 0.0 {
                decr_velocity.dont_decrease_x_when_pos();
            } else if increase.0 < 0.0 {
                decr_velocity.dont_decrease_x_when_neg();
            }
            if increase.1 > 0.0 {
                decr_velocity.dont_decrease_y_when_pos();
            } else if increase.1 < 0.0 {
                decr_velocity.dont_decrease_y_when_neg();
            }
            ai_data.is_moving = true;
            ai_data.velocity = increase;
            velocity.increase_with_max(increase, enemy.max_velocity);
        }
    }
}

fn run_for_turret_ai(
    dt: f32,
    player_data: &PlayerData,
    enemy: &Enemy,
    ai_data: &mut EnemyAiTurretData,
    transform: &Transform,
    // flipped: &Flipped,
    // entities: &Entities,
    // spritesheet_handles: &SpriteSheetHandles,
    // transforms: &mut WriteStorage<Transform>,
    // velocities: &mut WriteStorage<Velocity>,
    // sizes: &mut WriteStorage<Size>,
    // flippeds: &mut WriteStorage<Flipped>,
    // bullets: &mut WriteStorage<Bullet>,
    // collisions: &mut WriteStorage<Collision>,
    // check_collisions: &mut WriteStorage<CheckCollision>,
    // sprite_renders: &mut WriteStorage<SpriteRender>,
    // animations: &mut WriteStorage<Animation>,
    // transparents: &mut WriteStorage<Transparent>,
) {
    let enemy_pos = transform.translation();
    let distance_to_player = (
        enemy_pos.x - player_data.pos.0,
        enemy_pos.y - player_data.pos.1,
    );
    if enemy.in_trigger_distance(distance_to_player) {
        let now = Instant::now();
        if now - ai_data.last_shot_at
            >= Duration::from_millis(ai_data.shot_interval_ms)
        {
            // TODO
            println!("SHOOT BULLET");
            ai_data.last_shot_at = now;
            // Shoot bullet
            // let spritesheet_handle = spritesheet_handles
            //     .get("player_bullets")
            //     .expect("'player_bullets' spritesheet does not exist");
            // let entity = entities.create();
            // bullets
            //     .insert(
            //         entity,
            //         Bullet::new()
            //             .owner(BulletOwner::Enemy)
            //             .damage(enemy.damage)
            //             .lifetime(ai_data.bullet_lifetime)
            //             .build(),
            //     )
            //     .unwrap();
            // collisions.insert(entity, Collision::new()).unwrap();
            // check_collisions.insert(entity, CheckCollision).unwrap();
            // let mut transform = Transform::default();
            // transform.set_xyz(enemy_pos.x, enemy_pos.y, enemy_pos.z);
            // transforms.insert(entity, transform).unwrap();
            // velocities
            //     .insert(
            //         entity,
            //         Velocity::new(
            //             ai_data.bullet_velocity.0
            //                 * match flipped {
            //                     Flipped::None => 1.0,
            //                     Flipped::Horizontal => -1.0,
            //                     _ => 1.0,
            //                 },
            //             ai_data.bullet_velocity.1,
            //         ),
            //     )
            //     .unwrap();
            // sizes
            //     .insert(entity, Size::from(ai_data.bullet_size))
            //     .unwrap();
            // sprite_renders
            //     .insert(entity, SpriteRender {
            //         sprite_sheet:  spritesheet_handle.clone(),
            //         sprite_number: 0,
            //     })
            //     .unwrap();
            // animations
            //     .insert(
            //         entity,
            //         Animation::new()
            //             .default_sprite_sheet_handle(spritesheet_handle)
            //             .default_delay_ms(50)
            //             .sprite_ids(vec![0, 1, 2])
            //             .build(),
            //     )
            //     .unwrap();
            // transparents.insert(entity, Transparent).unwrap();
            // flippeds.insert(entity, flipped.clone()).unwrap();
        }
    }
}

struct PlayerData<'a> {
    pub player: &'a mut Player,
    pub pos:    Vector,
}
