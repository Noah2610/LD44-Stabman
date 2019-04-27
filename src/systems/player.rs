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
        ): Self::SystemData,
    ) {
        let dt = time.delta_seconds();

        for (mut player, mut velocity, decr_velocity_opt, player_collision) in (
            &mut players,
            &mut velocities,
            (&mut decr_velocities).maybe(),
            &collisions,
        )
            .join()
        {
            let sides_touching = SidesTouching::new(
                &entities,
                &player_collision,
                &collisions,
                &solids,
            );

            handle_move(
                dt,
                &input_handler,
                &player,
                &mut velocity,
                decr_velocity_opt,
                &sides_touching,
            )
        }
    }
}

fn handle_move(
    dt: f32,
    input_handler: &Read<InputHandler<String, String>>,
    player: &Player,
    velocity: &mut Velocity,
    mut decr_velocity_opt: Option<&mut DecreaseVelocity>,
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
            decr_velocity_opt.as_mut().map(|decr| {
                if x > 0.0 {
                    decr.dont_decrease_x_when_pos();
                } else if x < 0.0 {
                    decr.dont_decrease_x_when_neg();
                }
            });
        }
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
}
