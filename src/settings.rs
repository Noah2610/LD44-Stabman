use std::fmt;

use deathframe::geo::Vector;
use serde::de::{self, Visitor};
use serde::{Deserialize, Deserializer, Serialize};

pub mod prelude {
    pub use super::Settings;
    pub use super::SettingsCamera;
    pub use super::SettingsItem;
    pub use super::SettingsItems;
    pub use super::SettingsLevelManager;
    pub use super::SettingsLoadingText;
    pub use super::SettingsPlayer;
    pub use super::SettingsPlayerQuickTurnaround;
}

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub player:        SettingsPlayer,
    pub camera:        SettingsCamera,
    pub loading_text:  SettingsLoadingText,
    pub level_manager: SettingsLevelManager,
    pub enemies:       SettingsEnemies,
    pub items:         SettingsItems,
}

#[derive(Clone, Deserialize)]
pub struct SettingsCamera {
    pub size:       Vector,
    pub inner_size: Vector,
    pub base_speed: Vector,
    pub deadzone:   Vector,
}

#[derive(Clone, Deserialize)]
pub struct SettingsPlayer {
    pub size:                       Vector,
    pub acceleration:               Vector,
    pub jump_strength:              f32,
    pub wall_jump_strength:         Vector,
    pub decr_jump_strength:         f32,
    pub min_jump_velocity:          f32,
    pub max_velocity:               (Option<f32>, Option<f32>),
    pub decr_velocity:              Vector,
    pub gravity:                    Vector,
    pub jump_gravity:               Vector,
    pub slide_strength:             f32,
    pub quick_turnaround:           SettingsPlayerQuickTurnaround,
    pub air_quick_turnaround:       SettingsPlayerQuickTurnaround,
    pub decrease_x_velocity_in_air: bool,
    pub health:                     u32,
    pub damage:                     u32,
}

#[derive(Clone)]
pub enum SettingsPlayerQuickTurnaround {
    No,             // 0
    ResetVelocity,  // 1
    InvertVelocity, // 2
}

#[derive(Clone, Deserialize)]
pub struct SettingsLoadingText {
    pub text:      String,
    pub font_file: String,
    pub font_size: f32,
}

#[derive(Clone, Deserialize)]
pub struct SettingsLevelManager {
    pub levels_dir:  String,
    pub level_names: Vec<String>,
    pub tile_size:   Vector,
}

#[derive(Clone, Deserialize)]
pub struct SettingsEnemies {
    pub gravity: Vector,
    pub normal:  SettingsEnemy,
}

#[derive(Clone, Deserialize)]
pub struct SettingsEnemy {
    pub health:           u32,
    pub damage:           u32,
    pub reward:           u32,
    pub knockback:        Vector,
    pub trigger_distance: Vector,
    pub acceleration:     Vector,
    pub max_velocity:     (Option<f32>, Option<f32>),
    pub decr_velocity:    Vector,
}

#[derive(Clone, Deserialize)]
pub struct SettingsItems {
    pub extra_jump: SettingsItem,
    pub wall_jump:  SettingsItem,
}

#[derive(Clone, Deserialize)]
pub struct SettingsItem {
    pub cost: u32,
}

struct QTAVisitor;

impl<'de> Visitor<'de> for QTAVisitor {
    type Value = SettingsPlayerQuickTurnaround;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an integer between 0 and 2 (inclusive)")
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        use SettingsPlayerQuickTurnaround as QTA;
        match value {
            0 => Ok(QTA::No),
            1 => Ok(QTA::ResetVelocity),
            2 => Ok(QTA::InvertVelocity),
            _ => Err(E::custom(format!("Value out of range: {}", value))),
        }
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i8(value as i8)
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i8(value as i8)
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_i8(value as i8)
    }
}

impl<'de> Deserialize<'de> for SettingsPlayerQuickTurnaround {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_i32(QTAVisitor)
    }
}
