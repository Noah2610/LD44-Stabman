pub mod prelude {
    pub use super::Enemy;
    pub use super::EnemyAi;
    pub use super::EnemyType;
}

use std::time::{Duration, Instant};

use super::component_prelude::*;
use super::Player;
use crate::settings::SettingsEnemy;

#[derive(Clone, PartialEq)]
pub enum EnemyAi {
    Tracer,
}

impl Component for EnemyAi {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, PartialEq)]
pub enum EnemyType {
    Normal,
    Charger,
    Flying,
    Reaper,
}

pub struct Enemy {
    pub enemy_type:         EnemyType,
    pub health:             u32,
    pub damage:             u32,
    pub reward:             u32,
    pub knockback:          Vector,
    pub trigger_distance:   Vector,
    pub acceleration:       Vector,
    pub knockbacked_at:     Option<Instant>,
    pub knockback_duration: Duration,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, settings: SettingsEnemy) -> Self {
        Self {
            enemy_type:         enemy_type,
            health:             settings.health,
            damage:             settings.damage,
            reward:             settings.reward,
            knockback:          settings.knockback,
            trigger_distance:   settings.trigger_distance,
            acceleration:       settings.acceleration,
            knockbacked_at:     None,
            knockback_duration: Duration::from_millis(
                settings.knockback_duration_ms,
            ),
        }
    }

    pub fn deal_damage_to(&self, player: &mut Player) {
        player.take_damage(self.damage);
    }

    pub fn take_damage(&mut self, damage: u32) {
        if (self.health as i32) - (damage as i32) >= 0 {
            self.health -= damage;
        } else {
            self.health = 0;
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    pub fn in_trigger_distance(&self, distance: Vector) -> bool {
        distance.0.abs() <= self.trigger_distance.0.abs()
            || distance.1.abs() <= self.trigger_distance.1.abs()
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
