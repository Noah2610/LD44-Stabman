mod bullet;
mod dont_delete_on_next_level;
mod enemy;
mod goal;
mod harmable;
mod harmful;
mod heart;
mod hearts_container;
mod invincible;
mod item;
mod no_attack;
mod noclip;
mod player;
mod player_attack;
mod player_heart;
mod timer_ui;

pub mod prelude {
    pub use deathframe::components::prelude::*;

    pub use super::bullet::prelude::*;
    pub use super::dont_delete_on_next_level::DontDeleteOnNextLevel;
    pub use super::enemy::prelude::*;
    pub use super::goal::Goal;
    pub use super::harmable::Harmable;
    pub use super::harmful::Harmful;
    pub use super::heart::Heart;
    pub use super::hearts_container::prelude::*;
    pub use super::invincible::Invincible;
    pub use super::item::prelude::*;
    pub use super::no_attack::NoAttack;
    pub use super::noclip::Noclip;
    pub use super::player::Player;
    pub use super::player_attack::PlayerAttack;
    pub use super::player_heart::PlayerHeart;
    pub use super::timer_ui::{TimerType, TimerUi};
}

mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use super::enemy::Facing;
    pub use super::helpers::*;
    pub use crate::settings::prelude::*;
}

pub mod helpers {
    pub trait Health {
        fn health(&self) -> u32;
        fn health_mut(&mut self) -> &mut u32;
        fn take_damage(&mut self, damage: u32);
    }
}

pub use prelude::*;
