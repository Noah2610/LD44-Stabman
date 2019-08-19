mod menu;
mod resources;
mod stats;

pub use menu::*;
pub use resources::*;
pub use stats::*;

use amethyst::ui::{Anchor as AmethystAnchor, UiTransform};

#[derive(Clone)]
pub enum CampaignType {
    Normal,
    Bonus,
}

impl Default for CampaignType {
    fn default() -> Self {
        CampaignType::Normal
    }
}

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
