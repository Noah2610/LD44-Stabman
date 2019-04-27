use deathframe::geo::Vector;

pub mod prelude {
    pub use super::Settings;
    pub use super::SettingsCamera;
    pub use super::SettingsLevelManager;
    pub use super::SettingsLoadingText;
    pub use super::SettingsPlayer;
}

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub player:        SettingsPlayer,
    pub camera:        SettingsCamera,
    pub loading_text:  SettingsLoadingText,
    pub level_manager: SettingsLevelManager,
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
    pub size:          Vector,
    pub acceleration:  Vector,
    pub jump_strength: f32,
    pub max_velocity:  (Option<f32>, Option<f32>),
    pub decr_velocity: Vector,
    pub gravity:       Vector,
    pub jump_gravity:  Vector,
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
