use std::time::{Duration, Instant};

use super::system_prelude::*;

const DOUBLE_TAP_DASH: bool = false;

struct ActiveDash {
    dash_time:      u64,
    dash_direction: Direction,
}

#[derive(Default)]
pub struct PlayerDashSystem {
    active_dashes: Vec<ActiveDash>,
    last_action:   Option<(Direction, Instant)>,
}

impl<'a> System<'a> for PlayerDashSystem {
    type SystemData = (
        Read<'a, Time>,
        Read<'a, InputManager>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Gravity>,
    );

    fn run(
        &mut self,
        (time, input_manager, mut players, mut velocities, mut gravities): Self::SystemData,
    ) {
        if let Some((mut player, mut player_velocity, mut player_gravity_opt)) =
            (&mut players, &mut velocities, (&mut gravities).maybe())
                .join()
                .next()
        {
            self.handle_is_dashing(
                &time,
                &mut player,
                &mut player_velocity,
                &mut player_gravity_opt,
            );
            // NOTE: It's more fun if the player can activate another dash, while they are already
            // dashing. So this `handle_is_not_dashing` method should always run.
            self.handle_is_not_dashing(
                &input_manager,
                &mut player,
                &mut player_velocity,
                &mut player_gravity_opt,
            );
        }
    }
}

impl PlayerDashSystem {
    fn handle_is_dashing(
        &mut self,
        time: &Read<Time>,
        mut player: &mut Player,
        mut player_velocity: &mut Velocity,
        player_gravity_opt: &mut Option<&mut Gravity>,
    ) {
        let dash_duration_ms = player.items_data.dash.dash_duration_ms;
        let dt_ms = time.delta_time().as_millis() as u64;
        let mut dashes_to_remove = Vec::new();

        for (index, active_dash) in
            (&mut self.active_dashes).iter_mut().enumerate()
        {
            active_dash.dash_time += dt_ms;
            if active_dash.dash_time > dash_duration_ms {
                // Stop dash
                dashes_to_remove.push(index);
            } else {
                apply_dash_velocity(
                    &mut player,
                    &mut player_velocity,
                    active_dash.dash_direction,
                )
            }
        }

        for index in dashes_to_remove {
            self.active_dashes.remove(index);
        }

        if self.active_dashes.is_empty() {
            if let Some(gravity) = player_gravity_opt {
                gravity.enable();
            }
        }
    }

    fn handle_is_not_dashing(
        &mut self,
        input_manager: &InputManager,
        mut player: &mut Player,
        mut player_velocity: &mut Velocity,
        player_gravity_opt: &mut Option<&mut Gravity>,
    ) {
        // If player has used up all their dashes, we don't need to bother checking.
        if !player.has_dash() {
            if self.last_action.is_some() {
                self.last_action = None;
            }
            return;
        }

        let now = Instant::now();

        for check_direction in Direction::iter() {
            let action_name = check_direction.action();

            if input_manager.is_down(ACTION_DASH_TRIGGER)
                && input_manager.is_pressed(action_name)
            {
                // With double-tap dashing
                if DOUBLE_TAP_DASH {
                    if let Some((last_direction, last_action_at)) =
                        self.last_action
                    {
                        let delay_duration = Duration::from_millis(
                            player.items_data.dash.dash_input_delay_ms,
                        );
                        if now < last_action_at + delay_duration {
                            if check_direction == last_direction {
                                self.start_dash(
                                    &mut player,
                                    &mut player_velocity,
                                    player_gravity_opt,
                                    check_direction,
                                );
                                break;
                            }
                        }
                    }

                    self.last_action = Some((check_direction, now));
                    break;

                // Without double-tap dashing
                } else {
                    self.start_dash(
                        &mut player,
                        &mut player_velocity,
                        player_gravity_opt,
                        check_direction,
                    );
                    break;
                }
            }
        }
    }

    fn start_dash(
        &mut self,
        mut player: &mut Player,
        mut player_velocity: &mut Velocity,
        player_gravity_opt: &mut Option<&mut Gravity>,
        dashing_direction: Direction,
    ) {
        // If player has used up all their dashes, we don't need to bother checking.
        if !player.has_dash() {
            if self.last_action.is_some() {
                self.last_action = None;
            }
            return;
        }

        self.active_dashes.push(ActiveDash {
            dash_time:      0,
            dash_direction: dashing_direction,
        });
        player.items_data.dash.used_dashes += 1;
        if let Some(gravity) = player_gravity_opt {
            gravity.disable();
        }
        // Completely kill all velocity when dashing starts
        player_velocity.clear();
        apply_dash_velocity(
            &mut player,
            &mut player_velocity,
            dashing_direction,
        );
    }
}

fn apply_dash_velocity(
    player: &mut Player,
    player_velocity: &mut Velocity,
    dashing_direction: Direction,
) {
    // Apply a constant velocity
    let dash_velocity = player.items_data.dash.dash_velocity;
    let velocity = match dashing_direction {
        Direction::UpLeft => (Some(-dash_velocity.0), Some(dash_velocity.1)),
        Direction::UpRight => (Some(dash_velocity.0), Some(dash_velocity.1)),
        Direction::DownLeft => (Some(-dash_velocity.0), Some(-dash_velocity.1)),
        Direction::DownRight => (Some(dash_velocity.0), Some(-dash_velocity.1)),
        Direction::Up => (None, Some(dash_velocity.1)),
        Direction::Down => (None, Some(-dash_velocity.1)),
        Direction::Left => (Some(-dash_velocity.0), None),
        Direction::Right => (Some(dash_velocity.0), None),
    };
    velocity.0.map(|velx| player_velocity.x = velx);
    velocity.1.map(|vely| player_velocity.y = vely);
}
