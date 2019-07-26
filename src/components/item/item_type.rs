use amethyst::ecs::World;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};
use deathframe::handlers::SpriteSheetHandles;

use crate::resource_helpers::*;
use crate::settings::{SettingsItem, SettingsItems};

const SPRITESHEET_FILENAME: &str = "items.png";

#[derive(Clone, PartialEq)]
pub enum ItemType {
    ExtraJump,
    WallJump,
    Knockback,
    BulletShoot,
    Dash,
    SpeedUp,
    JumpUp,
    DamageUp,
}

impl ItemType {
    pub fn settings(&self, settings: &SettingsItems) -> SettingsItem {
        match self {
            ItemType::ExtraJump => settings.extra_jump.clone(),
            ItemType::WallJump => settings.wall_jump.clone(),
            ItemType::Knockback => settings.knockback.clone(),
            ItemType::BulletShoot => settings.bullet_shoot.clone(),
            ItemType::Dash => settings.dash.clone(),
            ItemType::SpeedUp => settings.speed_up.clone(),
            ItemType::JumpUp => settings.jump_up.clone(),
            ItemType::DamageUp => settings.damage_up.clone(),
        }
    }

    pub fn sprite_id(&self) -> usize {
        match self {
            ItemType::ExtraJump => 0,
            ItemType::WallJump => 1,
            ItemType::Knockback => 9,
            ItemType::BulletShoot => 7,
            ItemType::Dash => 8,
            ItemType::SpeedUp => 5,
            ItemType::JumpUp => 6,
            ItemType::DamageUp => 4,
        }
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
            "BulletShoot" => ItemType::BulletShoot,
            "Dash" => ItemType::Dash,
            "SpeedUp" => ItemType::SpeedUp,
            "JumpUp" => ItemType::JumpUp,
            "DamageUp" => ItemType::DamageUp,
            n => panic!(format!("Item '{}' does not exist", n)),
        }
    }
}
