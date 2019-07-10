mod bullet;
mod bullet_creator;
mod debug;
mod enemy_ai;
mod goal;
mod harmful;
mod health_display;
mod loader;
mod player_attack;
mod player_controls;
mod player_dash;
mod player_take_damage;

pub mod prelude {
    pub use deathframe::systems::prelude::*;

    pub use super::bullet::BulletSystem;
    pub use super::bullet_creator::BulletCreatorSystem;
    pub use super::debug::DebugSystem;
    pub use super::enemy_ai::EnemyAiSystem;
    pub use super::goal::GoalSystem;
    pub use super::harmful::HarmfulSystem;
    pub use super::health_display::HealthDisplaySystem;
    pub use super::loader::LoaderSystem;
    pub use super::player_attack::PlayerAttackSystem;
    pub use super::player_controls::PlayerControlsSystem;
    pub use super::player_dash::PlayerDashSystem;
    pub use super::player_take_damage::PlayerTakeDamageSystem;
}

mod system_prelude {
    pub use deathframe::geo::Side;
    pub use deathframe::handlers::SpriteSheetHandles;
    pub use deathframe::systems::system_prelude::*;

    pub use super::helpers::*;
    pub use crate::bullet_creator::prelude::*;
    pub use crate::components::helpers as component_helpers;
    pub use crate::components::prelude::*;
    pub use crate::solid_tag::SolidTag;
}

mod helpers;

pub use prelude::*;
