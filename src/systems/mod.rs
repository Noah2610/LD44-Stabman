mod debug;
mod goal;
mod health_display;
mod player_attack;
mod player_controls;
mod player_take_damage;

pub mod prelude {
    pub use deathframe::systems::prelude::*;

    pub use super::debug::DebugSystem;
    pub use super::goal::GoalSystem;
    pub use super::health_display::HealthDisplaySystem;
    pub use super::player_attack::PlayerAttackSystem;
    pub use super::player_controls::PlayerControlsSystem;
    pub use super::player_take_damage::PlayerTakeDamageSystem;
}

mod system_prelude {
    pub use deathframe::geo::Side;
    pub use deathframe::systems::system_prelude::*;

    pub use crate::components::prelude::*;
}

pub use prelude::*;
