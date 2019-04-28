use super::component_prelude::*;
use super::Enemy;

pub struct Player {
    pub acceleration:               Vector,
    pub air_acceleration:           Vector,
    pub jump_strength:              f32,
    pub decr_jump_strength:         f32,
    pub min_jump_velocity:          f32,
    pub max_velocity:               (Option<f32>, Option<f32>),
    pub gravity:                    Vector,
    pub jump_gravity:               Vector,
    pub slide_strength:             f32,
    pub quick_turnaround:           SettingsPlayerQuickTurnaround,
    pub air_quick_turnaround:       SettingsPlayerQuickTurnaround,
    pub decrease_x_velocity_in_air: bool,
    pub health:                     f32,
    pub damage:                     f32,
    pub is_attacking:               bool,
}

impl Player {
    pub fn new(settings: SettingsPlayer) -> Self {
        Self {
            acceleration:               settings.acceleration,
            air_acceleration:           settings.acceleration, // TODO
            jump_strength:              settings.jump_strength,
            decr_jump_strength:         settings.decr_jump_strength,
            min_jump_velocity:          settings.min_jump_velocity,
            max_velocity:               settings.max_velocity,
            gravity:                    settings.gravity,
            jump_gravity:               settings.jump_gravity,
            slide_strength:             settings.slide_strength,
            quick_turnaround:           settings.quick_turnaround,
            air_quick_turnaround:       settings.air_quick_turnaround,
            decrease_x_velocity_in_air: settings.decrease_x_velocity_in_air,
            health:                     settings.health,
            damage:                     settings.damage,
            is_attacking:               false,
        }
    }

    pub fn deal_damage_to(&self, enemy: &mut Enemy) {
        enemy.take_damage(self.damage);
    }

    pub fn take_damage(&mut self, damage: f32) {
        self.health -= damage;
    }

    pub fn gain_reward(&mut self, reward: f32) {
        self.health += reward;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0.0
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0.0
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}
