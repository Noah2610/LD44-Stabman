mod debug;

pub mod prelude {
    pub use deathframe::systems::prelude::*;

    pub use super::debug::DebugSystem;
}

mod system_prelude {
    pub use deathframe::systems::system_prelude::*;
}

pub use prelude::*;
