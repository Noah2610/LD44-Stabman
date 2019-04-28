mod player;
mod player_attack;

pub mod prelude {
    pub use deathframe::components::prelude::*;

    pub use super::player::Player;
    pub use super::player_attack::PlayerAttack;
}

mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use crate::settings::prelude::*;
}

pub use prelude::*;
