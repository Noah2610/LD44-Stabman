use super::system_prelude::*;

pub struct PlayerSystem;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Time>,
        Read<'a, InputHandler<String, String>>,
        Read<'a, InputManager>,
        ReadStorage<'a, Collision>,
        ReadStorage<'a, Solid>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, DecreaseVelocity>,
        WriteStorage<'a, Gravity>,
        WriteStorage<'a, AnimationsContainer>,
        WriteStorage<'a, Flipped>,
    );

    fn run(
        &mut self,
        (
            entities,
            time,
            input_handler,
            input_manager,
            collisions,
            solids,
            mut players,
            mut velocities,
            mut decr_velocities,
            mut gravities,
            mut animations_containers,
            mut flippeds,
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (
            player,
            velocity,
            decr_velocity,
            gravity,
            player_collision,
            animations_container,
            flipped_opt,
        ) in (
            &mut players,
            &mut velocities,
            &mut decr_velocities,
            &mut gravities,
            &collisions,
            &mut animations_containers,
            (&mut flippeds).maybe(),
        )
            .join()
        {
            let sides_touching = SidesTouching::new(
                &entities,
                player_collision,
                &collisions,
                &solids,
            );

            handle_wall_cling(
                &input_manager,
                player,
                velocity,
                &sides_touching,
            );

            handle_on_ground(player, velocity, &sides_touching);

            handle_move(
                dt,
                &input_handler,
                player,
                velocity,
                decr_velocity,
                animations_container,
                flipped_opt,
                &sides_touching,
            );

            handle_jump(
                &input_manager,
                player,
                velocity,
                gravity,
                &sides_touching,
            );

            handle_attack(&input_manager, player, animations_container);
        }
    }
}

fn handle_wall_cling(
    input_manager: &InputManager,
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
    flipped_opt: Option<&mut Flipped>,
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
            velocity.x += (if on_ground {
                player.acceleration.0
            } else {
                player.air_acceleration.0
            } * dt)
                * x_sign; // TODO: Maybe don't use the sign? Might work well with controller axis inputs.

            // Don't decrease velocity when moving
            if x > 0.0 {
                decr_velocity.dont_decrease_x_when_pos();
            } else if x < 0.0 {
                decr_velocity.dont_decrease_x_when_neg();
            }

            // Set walking animation
            animations_container.set("walking");
            // Flip animation
            if let Some(flip) = flipped_opt {
                if flip == &Flipped::Horizontal && x > 0.0 {
                    *flip = Flipped::None;
                } else if flip == &Flipped::None && x < 0.0 {
                    *flip = Flipped::Horizontal;
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
    if input_manager.is_down("player_jump") && sides_touching.is_touching_bottom
    {
        // Jump
        velocity.y += player.jump_strength;
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

/// Handle some specifics when player is standing on solid ground.
fn handle_on_ground(
    player: &mut Player,
    velocity: &mut Velocity,
    sides_touching: &SidesTouching,
) {
    // Reset y velocity to 0 when standing on solid ground
    // or when hitting a solid ceiling.
    if (sides_touching.is_touching_bottom && velocity.y < 0.0)
        || (sides_touching.is_touching_top && velocity.y > 0.0)
    {
        velocity.y = 0.0;
    }
}

fn handle_attack(
    input_manager: &InputManager,
    player: &mut Player,
    animations_container: &mut AnimationsContainer,
) {
    if input_manager.is_down("player_attack") {
        // Play attack animation
        animations_container.play("attack");
    }
}

#[derive(Default, Clone)]
struct SidesTouching {
    pub is_touching_top:    bool,
    pub is_touching_bottom: bool,
    pub is_touching_left:   bool,
    pub is_touching_right:  bool,
}

impl<'a> SidesTouching {
    pub fn new(
        entities: &Entities<'a>,
        player_collision: &Collision,
        collisions: &ReadStorage<'a, Collision>,
        solids: &ReadStorage<Solid>,
    ) -> Self {
        let mut is_touching_top = false;
        let mut is_touching_bottom = false;
        let mut is_touching_left = false;
        let mut is_touching_right = false;
        if player_collision.in_collision() {
            for (other_entity, _, _) in (entities, collisions, solids).join() {
                if let Some(colliding_with) =
                    player_collision.collision_with(other_entity.id())
                {
                    match colliding_with.side {
                        Side::Top => is_touching_top = true,
                        Side::Bottom => is_touching_bottom = true,
                        Side::Left => is_touching_left = true,
                        Side::Right => is_touching_right = true,
                        _ => (),
                    }
                    if is_touching_top
                        && is_touching_bottom
                        && is_touching_left
                        && is_touching_right
                    {
                        break;
                    }
                }
            }
        }
        Self {
            is_touching_top,
            is_touching_bottom,
            is_touching_left,
            is_touching_right,
        }
    }

    pub fn is_touching_horizontally(&self) -> bool {
        self.is_touching_left || self.is_touching_right
    }

    pub fn is_touching_vertically(&self) -> bool {
        self.is_touching_top || self.is_touching_bottom
    }
}