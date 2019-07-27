use std::time::{Duration, Instant};

use super::super::system_prelude::*;
use super::PlayerData;

pub(super) fn run(
    dt: f32,
    player_data: &PlayerData,
    enemy: &Enemy,
    ai_data: &mut EnemyAiTurretData,
    transform: &Transform,
    animations_container: &mut AnimationsContainer,
    bullet_creator: &mut BulletCreator,
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
            // Shoot bullet
            animations_container.play("shooting");
            ai_data.last_shot_at = now;
            bullet_creator.push(BulletComponents {
                bullet:    Bullet::new()
                    .owner(BulletOwner::Enemy)
                    .damage(enemy.damage)
                    .lifetime(ai_data.bullet_lifetime)
                    .knockback(enemy.knockback)
                    .facing(ai_data.facing.clone())
                    .build(),
                transform: {
                    let pos = transform.translation();
                    let mut trans = Transform::default();
                    trans.set_xyz(pos.x, pos.y, pos.z);
                    trans
                },
                velocity:  Velocity::new(
                    ai_data.bullet_velocity.0
                        * match ai_data.facing {
                            Facing::Right => 1.0,
                            Facing::Left => -1.0,
                        },
                    ai_data.bullet_velocity.1,
                ),
                size:      Size::from(ai_data.bullet_size),
            });
        }
    }
}
