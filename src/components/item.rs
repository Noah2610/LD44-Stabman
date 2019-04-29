use amethyst::ecs::World;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};
use deathframe::handlers::SpriteSheetHandles;

use super::component_prelude::*;
use super::Player;
use crate::resource_helpers::*;

const SPRITESHEET_FILENAME: &str = "items.png";

#[derive(Clone, PartialEq)]
pub enum Item {
    ExtraJump,
}

impl Item {
    pub fn apply(&self, player: &mut Player) {
        match self {
            Item::ExtraJump => {
                player.items_data.extra_jumps += 1;
            }
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

impl<T> From<T> for Item
where
    T: ToString,
{
    fn from(name: T) -> Self {
        match name.to_string().as_str() {
            "ExtraJump" => Item::ExtraJump,
            n => panic!(format!("Item '{}' does not exist", n)),
        }
    }
}

impl Component for Item {
    type Storage = VecStorage<Self>;
}
