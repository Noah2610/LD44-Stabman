pub mod prelude {
    pub use super::Enemy;
    pub use super::EnemyAi;
    pub use super::EnemyAiChargerData;
    pub use super::EnemyType;
}

use std::time::{Duration, Instant};

use deathframe::geo::Side;

use super::component_prelude::*;
use super::Player;
use crate::settings::SettingsEnemy;

// If player is within this range, enemy should _not_ move closer
// (to avoid alternating Flipped state, when crossing)
const TRIGGER_DISTANCE_DEADZONE: (f32, f32) = (16.0, 16.0);

#[derive(Clone, PartialEq, Default)]
pub struct EnemyAiChargerData {
    pub is_moving:                        bool,
    pub velocity:                         Vector,
    pub stop_moving_when_colliding_sides: Option<Vec<Side>>,
}

#[derive(Clone, PartialEq)]
pub enum EnemyAi {
    Tracer,
    Charger(EnemyAiChargerData),
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
    pub enemy_type:       EnemyType,
    pub health:           u32,
    pub damage:           u32,
    pub reward:           u32,
    pub knockback:        Vector,
    pub trigger_distance: Vector,
    pub acceleration:     Vector,
    pub max_velocity:     (Option<f32>, Option<f32>),
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, settings: SettingsEnemy) -> Self {
        Self {
            enemy_type:       enemy_type,
            health:           settings.health,
            damage:           settings.damage,
            reward:           settings.reward,
            knockback:        settings.knockback,
            trigger_distance: settings.trigger_distance,
            acceleration:     settings.acceleration,
            max_velocity:     settings.max_velocity,
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

    pub fn in_trigger_distance(&self, distance: (f32, f32)) -> bool {
        let distance = (distance.0.abs(), distance.1.abs());
        distance.0 <= self.trigger_distance.0.abs()
            && distance.1 <= self.trigger_distance.1.abs()
    }

    pub fn is_outside_deadzone_x(&self, distance: f32) -> bool {
        let distance = distance.abs();
        distance > TRIGGER_DISTANCE_DEADZONE.0
    }

    pub fn is_outside_deadzone_y(&self, distance: f32) -> bool {
        let distance = distance.abs();
        distance > TRIGGER_DISTANCE_DEADZONE.1
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
