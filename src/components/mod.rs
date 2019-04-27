mod player;

pub mod prelude {
    pub use deathframe::components::prelude::*;

    pub use super::player::Player;
}

mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use crate::settings::prelude::*;
}

pub use prelude::*;
