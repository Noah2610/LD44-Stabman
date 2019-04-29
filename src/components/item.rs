pub mod prelude {
    pub use super::Item;
    pub use super::ItemType;
}

use amethyst::ecs::World;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};
use deathframe::geo::Vector;
use deathframe::handlers::SpriteSheetHandles;

use super::component_prelude::*;
use super::Player;
use crate::resource_helpers::*;
use crate::settings::SettingsItems;

const SPRITESHEET_FILENAME: &str = "items.png";

pub struct Item {
    pub item_type: ItemType,
    pub cost:      u32,
}

impl Item {
    pub fn new<T>(name: T, items_settings: &SettingsItems) -> Self
    where
        T: ToString,
    {
        let item_type = ItemType::from(name);
        let settings = item_type.settings(items_settings);
        Self {
            item_type: item_type,
            cost:      settings.cost,
        }
    }

    pub fn apply(&self, player: &mut Player, settings: &SettingsItems) {
        match self.item_type {
            ItemType::ExtraJump => {
                player.items_data.extra_jumps += 1;
            }
            ItemType::WallJump => {
                player.items_data.can_wall_jump = true;
            }
            ItemType::Knockback => {
                player.items_data.has_knockback = true;
                player.items_data.knockback.0 +=
                    settings.settings.knockback_strength.0;
                player.items_data.knockback.1 +=
                    settings.settings.knockback_strength.1;
            }
        }
    }
}

impl Component for Item {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, PartialEq)]
pub enum ItemType {
    ExtraJump,
    WallJump,
    Knockback,
}

impl ItemType {
    pub fn settings(&self, settings: &SettingsItems) -> SettingsItem {
        match self {
            ItemType::ExtraJump => settings.extra_jump.clone(),
            ItemType::WallJump => settings.wall_jump.clone(),
            ItemType::Knockback => settings.knockback.clone(),
        }
    }

    pub fn sprite_id(&self) -> usize {
        // TODO
        0
    }

    pub fn sprite_sheet_handle_and_sprite_render(
        &self,
        world: &mut World,
    ) -> (SpriteSheetHandle, SpriteRender) {
        let mut spritesheet_handles =
            world.write_resource::<SpriteSheetHandles>();

        let handle = spritesheet_handles.get_or_load(
            resource(format!("spritesheets/{}", SPRITESHEET_FILENAME)),
            world,
        );
        (handle.clone(), SpriteRender {
            sprite_sheet:  handle,
            sprite_number: self.sprite_id(),
        })
    }
}

impl<T> From<T> for ItemType
where
    T: ToString,
{
    fn from(name: T) -> Self {
        match name.to_string().as_str() {
            "ExtraJump" => ItemType::ExtraJump,
            "WallJump" => ItemType::WallJump,
            "Knockback" => ItemType::Knockback,
            n => panic!(format!("Item '{}' does not exist", n)),
        }
    }
}
