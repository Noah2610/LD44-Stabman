mod normal;

pub mod prelude {
    pub use super::normal::NormalEnemy;
    pub use super::Enemy;
    pub use super::EnemyType;
}

use super::component_prelude::*;
use super::Player;
use crate::settings::SettingsEnemy;

pub enum EnemyType {
    Normal(normal::NormalEnemy),
}

pub struct Enemy {
    pub enemy_type: EnemyType,
    pub health:     f32,
    pub damage:     f32,
    pub reward:     f32,
    pub knockback:  Vector,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, settings: SettingsEnemy) -> Self {
        Self {
            enemy_type: enemy_type,
            health:     settings.health,
            damage:     settings.damage,
            reward:     settings.reward,
            knockback:  settings.knockback,
        }
    }

    pub fn deal_damage_to(&self, player: &mut Player) {
        player.take_damage(self.damage);
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
