mod ingame;
mod paused;
mod startup;
// mod main_menu;

pub mod prelude {
    pub use super::ingame::Ingame;
    pub use super::paused::Paused;
    pub use super::startup::Startup;
    // pub use super::main_menu::MainMenu;
    // pub use super::paused::Paused;
}

mod state_prelude {
    pub use amethyst::assets::{AssetStorage, Loader};
    pub use amethyst::ecs::{Entity, World};
    pub use amethyst::input::is_close_requested;
    pub use amethyst::prelude::*;
    pub use amethyst::renderer::{
        Camera as AmethystCamera,
        DisplayConfig,
        PngFormat,
        Projection,
        SpriteRender,
        SpriteSheet,
        SpriteSheetFormat,
        SpriteSheetHandle,
        Texture,
        TextureMetadata,
        Transparent,
        VirtualKeyCode,
    };
    pub use amethyst::ui::{
        Anchor as AmethystAnchor,
        TtfFormat,
        UiCreator,
        UiText,
        UiTransform,
    };
    pub use amethyst::{State, StateData, StateEvent, Trans};

    pub use deathframe::custom_game_data::prelude::*;
    pub use deathframe::handlers::prelude::*;
    pub use deathframe::input_manager::InputManager;

    pub use super::helpers::*;
    pub use super::prelude::*;
    pub use crate::bullet_creator::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::resource_helpers::*;
    pub use crate::settings::prelude::*;
    pub use crate::world_helpers::*;
    pub use crate::CustomData;
}

pub use prelude::*;

mod helpers {
    use amethyst::ui::{Anchor as AmethystAnchor, UiTransform};

    /// `UiTransform::new` wrapper
    pub fn new_ui_transform<T: ToString>(
        name: T,
        anchor: AmethystAnchor,
        pos: (f32, f32, f32, f32, f32, i32),
    ) -> UiTransform {
        UiTransform::new(
            name.to_string(),
            anchor,
            pos.0, // x
            pos.1, // y
            pos.2, // z
            pos.3, // width
            pos.4, // height
            pos.5, // tab-order (?)
        )
    }
}
