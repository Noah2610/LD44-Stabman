mod bullet;
mod bullet_creator;
mod debug;
mod enemy_ai;
mod goal;
mod harmful;
mod health_display;
mod hearts_system;
mod loader;
mod noclip;
mod player_attack;
mod player_controls;
mod player_dash;
mod player_take_damage;
mod sync_hearts_containers_with_health;
mod timer;

pub mod prelude {
    pub use deathframe::systems::prelude::*;

    pub use super::bullet::BulletSystem;
    pub use super::bullet_creator::BulletCreatorSystem;
    pub use super::debug::DebugSystem;
    pub use super::enemy_ai::EnemyAiSystem;
    pub use super::goal::GoalSystem;
    pub use super::harmful::HarmfulSystem;
    pub use super::health_display::HealthDisplaySystem;
    pub use super::hearts_system::HeartsSystem;
    pub use super::loader::LoaderSystem;
    pub use super::noclip::NoclipSystem;
    pub use super::player_attack::PlayerAttackSystem;
    pub use super::player_controls::PlayerControlsSystem;
    pub use super::player_dash::PlayerDashSystem;
    pub use super::player_take_damage::PlayerTakeDamageSystem;
    pub use super::sync_hearts_containers_with_health::SyncHeartsContainersWithHealthSystem;
    pub use super::timer::TimerSystem;
}

mod system_prelude {
    pub use amethyst::ecs::World;
    pub use amethyst::renderer::DebugLines;
    pub use deathframe::geo::Side;
    pub use deathframe::handlers::SpriteSheetHandles;
    pub use deathframe::systems::system_prelude::*;

    pub use super::helpers::*;
    pub use crate::bullet_creator::prelude::*;
    pub use crate::components::helpers as component_helpers;
    pub use crate::components::prelude::*;
    pub use crate::settings::prelude::*;
    pub use crate::solid_tag::SolidTag;
    pub use crate::states::helpers::*;
}

mod helpers;

pub use prelude::*;
