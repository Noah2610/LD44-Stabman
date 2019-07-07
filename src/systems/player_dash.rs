use std::time::{Duration, Instant};

use super::system_prelude::*;

#[derive(Default)]
pub struct PlayerDashSystem {
    current_dash_time: u64,
    dashing_direction: Option<Direction>,
    last_action:       Option<(Direction, Instant)>,
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
        if let Some((mut player, mut player_velocity, player_gravity_opt)) =
            (&mut players, &mut velocities, (&mut gravities).maybe())
                .join()
                .next()
        {
            if let Some(dashing_direction) = self.dashing_direction {
                self.handle_is_dashing(
                    &time,
                    &mut player,
                    &mut player_velocity,
                    player_gravity_opt,
                    dashing_direction,
                );
            } else {
                self.handle_is_not_dashing(
                    &input_manager,
                    &mut player,
                    &mut player_velocity,
                    player_gravity_opt,
                );
            }
        }
    }
}

impl PlayerDashSystem {
    fn handle_is_dashing(
        &mut self,
        time: &Read<Time>,
        mut player: &mut Player,
        mut player_velocity: &mut Velocity,
        player_gravity_opt: Option<&mut Gravity>,
        dashing_direction: Direction,
    ) {
        let dash_duration_ms = player.items_data.dash.dash_duration_ms;
        let dt_ms = time.delta_time().as_millis() as u64;
        self.current_dash_time += dt_ms;
        if self.current_dash_time > dash_duration_ms {
            self.stop_dashing(player_gravity_opt);
        } else {
            self.apply_dash_velocity(
                &mut player,
                &mut player_velocity,
                dashing_direction,
            )
        }
    }

    fn handle_is_not_dashing(
        &mut self,
        input_manager: &InputManager,
        mut player: &mut Player,
        mut player_velocity: &mut Velocity,
        player_gravity_opt: Option<&mut Gravity>,
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

            if input_manager.is_down(action_name) {
                if let Some((last_direction, last_action_at)) = self.last_action
                {
                    let delay_duration = Duration::from_millis(
                        player.items_data.dash.dash_input_delay_ms,
                    );
                    if now < last_action_at + delay_duration {
                        if check_direction == last_direction {
                            self.start_dashing(
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
            }
        }
    }

    fn start_dashing(
        &mut self,
        mut player: &mut Player,
        mut player_velocity: &mut Velocity,
        player_gravity_opt: Option<&mut Gravity>,
        dashing_direction: Direction,
    ) {
        self.last_action = None;
        self.current_dash_time = 0;
        self.dashing_direction = Some(dashing_direction);
        player.items_data.dash.used_dashes += 1;
        if let Some(gravity) = player_gravity_opt {
            gravity.disable();
        }
        self.apply_dash_velocity(
            &mut player,
            &mut player_velocity,
            dashing_direction,
        );
    }

    fn stop_dashing(&mut self, player_gravity_opt: Option<&mut Gravity>) {
        self.last_action = None;
        self.dashing_direction = None;
        self.current_dash_time = 0;
        if let Some(gravity) = player_gravity_opt {
            gravity.enable();
        }
    }

    fn apply_dash_velocity(
        &self,
        player: &mut Player,
        player_velocity: &mut Velocity,
        dashing_direction: Direction,
    ) {
        // Apply a constant velocity
        let dash_velocity = player.items_data.dash.dash_velocity;
        let velocity = match dashing_direction {
            Direction::Up => (None, Some(dash_velocity.1)),
            Direction::Down => (None, Some(-dash_velocity.1)),
            Direction::Left => (Some(-dash_velocity.0), None),
            Direction::Right => (Some(dash_velocity.0), None),
        };
        velocity.0.map(|velx| player_velocity.x = velx);
        velocity.1.map(|vely| player_velocity.y = vely);
    }
}
