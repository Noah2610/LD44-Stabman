mod normal;

pub mod prelude {
    pub use super::normal::NormalEnemy;
    pub use super::Enemy;
    pub use super::EnemyType;
}

use super::component_prelude::*;
use crate::settings::SettingsEnemy;

pub enum EnemyType {
    Normal(normal::NormalEnemy),
}

pub struct Enemy {
    pub enemy_type: EnemyType,
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, settings: SettingsEnemy) -> Self {
        Self { enemy_type }
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
