use deathframe::handlers::SpriteSheetHandles;

use super::system_prelude::*;
use crate::settings::prelude::*;

pub struct PlayerControlsSystem;

impl<'a> System<'a> for PlayerControlsSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Settings>,
        ReadExpect<'a, SpriteSheetHandles>,
        Read<'a, Time>,
        Read<'a, InputHandler<String, String>>,
        Read<'a, InputManager>,
        ReadStorage<'a, Solid>,
        ReadStorage<'a, Goal>,
        ReadStorage<'a, Item>,
        WriteStorage<'a, Collision>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, DecreaseVelocity>,
        WriteStorage<'a, Gravity>,
        WriteStorage<'a, AnimationsContainer>,
        WriteStorage<'a, Flipped>,
        WriteStorage<'a, Bullet>,
        WriteStorage<'a, CheckCollision>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, SpriteRender>,
        WriteStorage<'a, Animation>,
        WriteStorage<'a, Transparent>,
    );

    fn run(
        &mut self,
        (
            entities,
            settings,
            spritesheet_handles,
            time,
            input_handler,
            input_manager,
            solids,
            goals,
            items,
            mut collisions,
            mut players,
            mut transforms,
            mut velocities,
            mut decr_velocities,
            mut gravities,
            mut animations_containers,
            mut flippeds,
            mut bullets,
            mut check_collisions,
            mut sizes,
            mut sprite_renders,
            mut animations,
            mut transparents,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        let goal_next_level = (&goals)
            .join()
            .next()
            .map(|goal| goal.next_level)
            .unwrap_or(false);

        let mut should_shoot_bullet = false;

        for (
            player,
            transform,
            velocity,
            decr_velocity,
            gravity,
            player_collision,
            animations_container,
            flipped,
        ) in (
            &mut players,
            &transforms,
            &mut velocities,
            &mut decr_velocities,
            &mut gravities,
            &collisions,
            &mut animations_containers,
            &mut flippeds,
        )
            .join()
        {
            let sides_touching = SidesTouching::new_with_collisions_mut(
                &entities,
                player_collision,
                &collisions,
                &solids,
            );

            handle_wall_cling(player, velocity, &sides_touching);

            handle_on_ground_and_in_air(
                player,
                velocity,
                decr_velocity,
                &sides_touching,
            );

            // Kill the player, if they fall below this y level
            if transform.translation().y <= settings.player.death_floor {
                player.health = 0;
            }

            if player.in_control && !goal_next_level {
                handle_move(
                    dt,
                    &input_handler,
                    player,
                    velocity,
                    decr_velocity,
                    animations_container,
                    flipped,
                    &sides_touching,
                );

                handle_jump(
                    &input_manager,
                    player,
                    velocity,
                    gravity,
                    &sides_touching,
                );

                should_shoot_bullet = handle_attack(
                    &input_manager,
                    player,
                    animations_container,
                    flipped,
                ) && player.items_data.can_shoot;

                handle_item_purchase(
                    &settings.items,
                    &entities,
                    &input_manager,
                    player,
                    player_collision,
                    &items,
                    &collisions,
                );
            } else if !player.in_control && !goal_next_level {
                // Start of a level
                // Play the level_start animation once, then regain control
                // TODO: Cleanup
                // animations_container.set("level_start");
                // if animations_container
                //     .current
                //     .as_ref()
                //     .map(|(_, anim)| anim.has_played())
                //     .unwrap_or(true)
                if true {
                    player.in_control = true;
                    animations_container.set("idle");
                }
            }
        }

        // Player BulletShoot
        if should_shoot_bullet {
            shoot_bullet(
                &entities,
                &spritesheet_handles,
                &players,
                &mut transforms,
                &mut velocities,
                &mut sizes,
                &mut flippeds,
                &mut bullets,
                &mut collisions,
                &mut check_collisions,
                &mut sprite_renders,
                &mut animations,
                &mut transparents,
            );
        }
    }
}

fn handle_wall_cling(
    player: &mut Player,
    velocity: &mut Velocity,
    sides_touching: &SidesTouching,
) {
    if sides_touching.is_touching_horizontally() {
        // Reset x velocity to 0 when colliding with a solid, horizontally.
        if (sides_touching.is_touching_left && velocity.x < 0.0)
            || (sides_touching.is_touching_right && velocity.x > 0.0)
        {
            velocity.x = 0.0;
        }
        // Clinging to wall, when not touching a solid vertically.
        if !sides_touching.is_touching_vertically() {
            // Keep y velocity at a constant velocity; slide down solid.
            let slide_strength = -player.slide_strength;
            if velocity.y < slide_strength {
                velocity.y = slide_strength;
            }
            // Reset ExtraJumps
            if player.items_data.used_extra_jumps != 0 {
                player.items_data.used_extra_jumps = 0;
            }
        }
    }
}

fn handle_move(
    dt: f32,
    input_handler: &InputHandler<String, String>,
    player: &Player,
    velocity: &mut Velocity,
    decr_velocity: &mut DecreaseVelocity,
    animations_container: &mut AnimationsContainer,
    flipped: &mut Flipped,
    sides_touching: &SidesTouching,
) {
    if let Some(x) = input_handler.axis_value("player_x") {
        use crate::settings::SettingsPlayerQuickTurnaround as QTA;

        let x = x as f32;
        if x != 0.0 {
            let x_sign = x.signum();
            let on_ground = sides_touching.is_touching_bottom;

            // Turnaround stuff
            let turned_around = x_sign != velocity.x.signum();
            if turned_around {
                // Quick turnaround, when on ground
                let qta_setting = if on_ground {
                    &player.quick_turnaround
                // Quick turnaround, when in air
                } else {
                    &player.air_quick_turnaround
                };
                match &qta_setting {
                    QTA::ResetVelocity => velocity.x = 0.0,
                    QTA::InvertVelocity => velocity.x *= -1.0,
                    _ => (),
                }
            }

            // Move player
            let velocity_increase = (if on_ground {
                player.acceleration.0
            } else {
                player.air_acceleration.0
            } * dt)
                * x_sign; // TODO: Maybe don't use the sign? Might work well with controller axis inputs.

            // Increase velocity with a maximum
            velocity
                .increase_x_with_max(velocity_increase, player.max_velocity.0);

            // Don't decrease velocity when moving
            if x > 0.0 {
                decr_velocity.dont_decrease_x_when_pos();
            } else if x < 0.0 {
                decr_velocity.dont_decrease_x_when_neg();
            }

            // Set walking animation
            animations_container.set("walking");
            // Flip animation
            if !player.is_attacking {
                if flipped == &Flipped::Horizontal && x > 0.0 {
                    *flipped = Flipped::None;
                } else if flipped == &Flipped::None && x < 0.0 {
                    *flipped = Flipped::Horizontal;
                }
            }
        } else {
            // Standing still - set idle animation
            animations_container.set("idle");
        }
    }
}

fn handle_jump(
    input_manager: &InputManager,
    player: &mut Player,
    velocity: &mut Velocity,
    gravity: &mut Gravity,
    sides_touching: &SidesTouching,
) {
    let jump_btn_down = input_manager.is_down("player_jump");
    let can_wall_jump = player.items_data.can_wall_jump
        && jump_btn_down
        && sides_touching.is_touching_horizontally();
    let can_jump = jump_btn_down
        && (sides_touching.is_touching_bottom || player.has_extra_jump())
        && !can_wall_jump;
    let mut jumped = false;
    if can_jump {
        if velocity.y < 0.0 {
            velocity.y = 0.0;
        }
        // Jump
        velocity.y += player.jump_strength;
        // Was an extra jump
        if !sides_touching.is_touching_bottom {
            player.items_data.used_extra_jumps += 1;
        }
        jumped = true;
    } else if can_wall_jump {
        if velocity.y < 0.0 {
            velocity.y = 0.0;
        }
        // Wall jump
        velocity.y += player.wall_jump_strength.1;
        if sides_touching.is_touching_left {
            velocity.x += player.wall_jump_strength.0;
        } else if sides_touching.is_touching_right {
            velocity.x -= player.wall_jump_strength.0;
        }
        jumped = true;
    }

    if jumped {
        // Set different gravity when jumping
        gravity.x = player.jump_gravity.0;
        gravity.y = player.jump_gravity.1;
    } else if input_manager.is_up("player_jump") {
        // Kill some of the upwards momentum, keeping at least a certain minimum velocity
        if velocity.y > player.decr_jump_strength {
            velocity.y = (velocity.y - player.decr_jump_strength)
                .max(player.min_jump_velocity);
        }
        // Set default gravity
        gravity.x = player.gravity.0;
        gravity.y = player.gravity.1;
    }
}

/// Handle some specifics when player is standing on solid ground vs when in air.
fn handle_on_ground_and_in_air(
    player: &mut Player,
    velocity: &mut Velocity,
    decr_velocity: &mut DecreaseVelocity,
    sides_touching: &SidesTouching,
) {
    // Reset y velocity to 0 when standing on solid ground
    // or when hitting a solid ceiling.
    if (sides_touching.is_touching_bottom && velocity.y < 0.0)
        || (sides_touching.is_touching_top && velocity.y > 0.0)
    {
        velocity.y = 0.0;
    }
    // Don't decrease velocity when in air.
    if !player.decrease_x_velocity_in_air && !sides_touching.is_touching_bottom
    {
        decr_velocity.dont_decrease_x();
    }
    // Recharge double jump
    if sides_touching.is_touching_bottom {
        if player.items_data.used_extra_jumps != 0 {
            player.items_data.used_extra_jumps = 0;
        }
    }
}

/// Returns `true` if the player started an attack
fn handle_attack<'a>(
    input_manager: &InputManager,
    player: &mut Player,
    animations_container: &mut AnimationsContainer,
    flipped: &mut Flipped,
) -> bool {
    let is_attacking = if !player.is_attacking {
        if input_manager.is_down("player_attack") {
            true
        } else if input_manager.is_down("player_attack_left") {
            *flipped = Flipped::Horizontal;
            true
        } else if input_manager.is_down("player_attack_right") {
            *flipped = Flipped::None;
            true
        } else {
            false
        }
    } else {
        false
    };

    if is_attacking {
        player.is_attacking = true;
        // Play attack animation
        animations_container.play("attack");
    }

    is_attacking
}

fn handle_item_purchase<'a>(
    settings: &SettingsItems,
    entities: &Entities<'a>,
    input_manager: &InputManager,
    player: &mut Player,
    player_collision: &Collision,
    items: &ReadStorage<'a, Item>,
    collisions: &WriteStorage<'a, Collision>,
) {
    for (item_entity, item, item_collision) in
        (entities, items, collisions).join()
    {
        let item_id = item_entity.id();
        if let Some(collision::Data {
            state: collision::State::Steady,
            ..
        }) = player_collision.collision_with(item_id)
        {
            if input_manager.is_down("player_buy_item") {
                // Pickup item
                item.apply(player, settings);
                entities.delete(item_entity).unwrap();
                player.take_damage(item.cost);
            }
        }
    }
}

fn shoot_bullet<'a>(
    entities: &Entities<'a>,
    spritesheet_handles: &SpriteSheetHandles,
    players: &WriteStorage<'a, Player>,
    transforms: &mut WriteStorage<'a, Transform>,
    velocities: &mut WriteStorage<'a, Velocity>,
    sizes: &mut WriteStorage<'a, Size>,
    flippeds: &mut WriteStorage<'a, Flipped>,
    bullets: &mut WriteStorage<'a, Bullet>,
    collisions: &mut WriteStorage<'a, Collision>,
    check_collisions: &mut WriteStorage<'a, CheckCollision>,
    sprite_renders: &mut WriteStorage<'a, SpriteRender>,
    animations: &mut WriteStorage<'a, Animation>,
    transparents: &mut WriteStorage<'a, Transparent>,
) {
    let player_data_opt = (players, &*transforms, &*flippeds)
        .join()
        .next()
        .map(|(player, transform, flipped)| {
            let trans = transform.translation();
            (player, (trans.x, trans.y, trans.z), flipped)
        });

    if let Some((player, player_pos, player_flipped)) = player_data_opt {
        let spritesheet_handle = spritesheet_handles
            .get("player_bullets")
            .expect("'player_bullets' spritesheet does not exist");
        let entity = entities.create();
        bullets
            .insert(
                entity,
                Bullet::new()
                    .owner(BulletOwner::Player)
                    .damage(player.items_data.bullet_damage)
                    .lifetime(player.items_data.bullet_lifetime)
                    .build(),
            )
            .unwrap();
        collisions.insert(entity, Collision::new()).unwrap();
        check_collisions.insert(entity, CheckCollision).unwrap();
        let mut transform = Transform::default();
        transform.set_xyz(player_pos.0, player_pos.1, player_pos.2);
        transforms.insert(entity, transform).unwrap();
        velocities
            .insert(
                entity,
                Velocity::new(
                    player.items_data.bullet_velocity.0
                        * match player_flipped {
                            Flipped::None => 1.0,
                            Flipped::Horizontal => -1.0,
                            _ => 1.0,
                        },
                    player.items_data.bullet_velocity.1,
                ),
            )
            .unwrap();
        sizes
            .insert(entity, Size::from(player.items_data.bullet_size))
            .unwrap();
        sprite_renders
            .insert(entity, SpriteRender {
                sprite_sheet:  spritesheet_handle.clone(),
                sprite_number: 0,
            })
            .unwrap();
        animations
            .insert(
                entity,
                Animation::new()
                    .default_sprite_sheet_handle(spritesheet_handle)
                    .default_delay_ms(50)
                    .sprite_ids(vec![0, 1, 2])
                    .build(),
            )
            .unwrap();
        transparents.insert(entity, Transparent).unwrap();
        flippeds.insert(entity, player_flipped.clone()).unwrap();
    }
}
