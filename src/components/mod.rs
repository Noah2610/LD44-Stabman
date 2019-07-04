mod bullet;
mod enemy;
mod goal;
mod heart;
mod item;
mod player;
mod player_attack;

pub mod prelude {
    pub use deathframe::components::prelude::*;

    pub use super::bullet::prelude::*;
    pub use super::enemy::prelude::*;
    pub use super::goal::Goal;
    pub use super::heart::Heart;
    pub use super::item::prelude::*;
    pub use super::player::Player;
    pub use super::player_attack::PlayerAttack;
}

mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use super::enemy::Facing;
    pub use crate::settings::prelude::*;
}

pub use prelude::*;
