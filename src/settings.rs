use deathframe::geo::Vector;

pub mod prelude {
    pub use super::Settings;
}

#[derive(Clone, Deserialize)]
pub struct Settings {
    pub player: SettingsPlayer,
    pub camera: SettingsCamera,
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
