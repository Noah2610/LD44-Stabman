use super::component_prelude::*;

pub struct Player {
    pub acceleration:         Vector,
    pub air_acceleration:     Vector,
    pub max_velocity:         (Option<f32>, Option<f32>),
    pub quick_turnaround:     SettingsPlayerQuickTurnaround,
    pub air_quick_turnaround: SettingsPlayerQuickTurnaround,
}

impl Player {
    pub fn new(settings: SettingsPlayer) -> Self {
        Self {
            acceleration:         settings.acceleration,
            air_acceleration:     settings.acceleration, // TODO
            max_velocity:         settings.max_velocity,
            quick_turnaround:     settings.quick_turnaround,
            air_quick_turnaround: settings.air_quick_turnaround,
        }
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
