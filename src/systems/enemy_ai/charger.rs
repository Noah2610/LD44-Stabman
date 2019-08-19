use deathframe::components::solid::SolidTag as _;
use deathframe::geo::Vector;

use super::super::system_prelude::*;
use super::PlayerData;

/// When the enemy sees the player, start moving in their direction;
/// don't stop moving until the enemy hits a wall.
pub(super) fn run(
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
                if let Some(collision::Data {
                    side,
                    state: collision::State::Enter,
                    ..
                }) = collision.collision_with(entity.id())
                {
                    solid.tag.collides_with(&other_solid.tag)
                        && stop_moving_sides.contains(side)
                } else {
                    false
                }
            })
        } else {
            false
        };
        if in_collision {
            // Stop moving
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
