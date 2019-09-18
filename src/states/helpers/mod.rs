mod menu;
mod stats;

pub use menu::*;
pub use stats::*;

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
