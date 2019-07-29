mod item_type;

pub mod prelude {
    pub use super::Item;
    pub use super::ItemType;
}

use std::time::Duration;

use super::component_prelude::*;
use super::Player;
use crate::settings::SettingsItems;

pub use item_type::ItemType;

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
                player.items_data.extra_jump.extra_jumps += 1;
            }
            ItemType::WallJump => {
                player.items_data.wall_jump.can_wall_jump = true;
            }
            ItemType::Knockback => {
                player.items_data.knockback.has_knockback = true;
                player.items_data.knockback.velocity.0 +=
                    settings.settings.knockback_strength.0;
                player.items_data.knockback.velocity.1 +=
                    settings.settings.knockback_strength.1;
            }
            ItemType::BulletShoot => {
                player.items_data.bullet_shoot.can_shoot = true;
                player.items_data.bullet_shoot.damage =
                    settings.settings.bullet_shoot_damage;
                player.items_data.bullet_shoot.velocity =
                    settings.settings.bullet_shoot_velocity;
                player.items_data.bullet_shoot.size =
                    settings.settings.bullet_shoot_size;
                player.items_data.bullet_shoot.lifetime = Duration::from_millis(
                    settings.settings.bullet_shoot_lifetime_ms,
                );
            }
            ItemType::Dash => {
                player.items_data.dash.dashes += 1;
                player.items_data.dash.duration_ms =
                    settings.settings.dash_duration_ms;
                player.items_data.dash.velocity =
                    settings.settings.dash_velocity;
                player.items_data.dash.input_delay_ms =
                    settings.settings.dash_input_delay_ms;
                player.items_data.dash.double_tap =
                    settings.settings.dash_double_tap;
            }
            ItemType::BulletDeflect => {
                player.items_data.bullet_deflect.can_deflect = true;
                player.items_data.bullet_deflect.damage =
                    settings.settings.bullet_deflect_damage;
                player.items_data.bullet_deflect.velocity_mult =
                    settings.settings.bullet_deflect_velocity_mult;
                player.items_data.bullet_deflect.lifetime =
                    Duration::from_millis(
                        settings.settings.bullet_deflect_lifetime_ms,
                    );
            }
            ItemType::Thrust => {
                player.items_data.thrust.can_thrust = true;
                player.items_data.thrust.strength.0 +=
                    settings.settings.thrust_strength.0;
                player.items_data.thrust.strength.1 +=
                    settings.settings.thrust_strength.1;
            }
            ItemType::SpeedUp => {
                player
                    .max_velocity
                    .0
                    .as_mut()
                    .map(|x| *x += settings.settings.speed_up_max_velocity_up);
            }
            ItemType::JumpUp => {
                player.jump_strength += settings.settings.jump_up;
            }
            ItemType::DamageUp => {
                player.damage += settings.settings.damage_up;
            }
        }
    }
}

impl Component for Item {
    type Storage = VecStorage<Self>;
}
