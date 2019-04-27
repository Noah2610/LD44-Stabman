use super::component_prelude::*;

pub struct Player {
    pub acceleration:         Vector,
    pub air_acceleration:     Vector,
    pub jump_strength:        f32,
    pub decr_jump_strength:   f32,
    pub min_jump_velocity:    f32,
    pub max_velocity:         (Option<f32>, Option<f32>),
    pub gravity:              Vector,
    pub jump_gravity:         Vector,
    pub slide_strength:       f32,
    pub quick_turnaround:     SettingsPlayerQuickTurnaround,
    pub air_quick_turnaround: SettingsPlayerQuickTurnaround,
}

impl Player {
    pub fn new(settings: SettingsPlayer) -> Self {
        Self {
            acceleration:         settings.acceleration,
            air_acceleration:     settings.acceleration, // TODO
            jump_strength:        settings.jump_strength,
            decr_jump_strength:   settings.decr_jump_strength,
            min_jump_velocity:    settings.min_jump_velocity,
            max_velocity:         settings.max_velocity,
            gravity:              settings.gravity,
            jump_gravity:         settings.jump_gravity,
            slide_strength:       settings.slide_strength,
            quick_turnaround:     settings.quick_turnaround,
            air_quick_turnaround: settings.air_quick_turnaround,
        }
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
