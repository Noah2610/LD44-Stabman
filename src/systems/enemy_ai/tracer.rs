use super::super::system_prelude::*;
use super::PlayerData;
use deathframe::geo::Vector;

/// Simply move towards the player, when they are within trigger distance.
pub(super) fn run<'a>(
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
