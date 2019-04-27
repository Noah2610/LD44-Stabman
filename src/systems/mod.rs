mod debug;
mod player;

pub mod prelude {
    pub use deathframe::systems::prelude::*;

    pub use super::debug::DebugSystem;
    pub use super::player::PlayerSystem;
}

mod system_prelude {
    pub use deathframe::geo::Side;
    pub use deathframe::systems::system_prelude::*;

    pub use crate::components::prelude::*;
}

pub use prelude::*;
