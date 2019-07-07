use std::time::Duration;

use super::component_prelude::*;
use super::Enemy;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemsData {
    pub extra_jumps:      u32,
    pub used_extra_jumps: u32,
    pub can_wall_jump:    bool,
    pub knockback:        Vector,
    pub has_knockback:    bool,
    pub can_shoot:        bool,
    pub bullet_damage:    u32,
    pub bullet_velocity:  Vector,
    pub bullet_size:      Vector,
    pub bullet_lifetime:  Duration,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub acceleration:               Vector,
    pub air_acceleration:           Vector,
    pub jump_strength:              f32,
    pub wall_jump_strength:         Vector,
    pub decr_jump_strength:         f32,
    pub min_jump_velocity:          f32,
    pub max_velocity:               (Option<f32>, Option<f32>),
    pub gravity:                    Vector,
    pub jump_gravity:               Vector,
    pub slide_strength:             f32,
    pub quick_turnaround:           SettingsPlayerQuickTurnaround,
    pub air_quick_turnaround:       SettingsPlayerQuickTurnaround,
    pub decrease_x_velocity_in_air: bool,
    pub health:                     u32,
    pub damage:                     u32,
    pub is_attacking:               bool,
    pub in_control:                 bool,
    pub items_data:                 ItemsData,
}

impl Player {
    pub fn new(settings: SettingsPlayer) -> Self {
        Self {
            acceleration:               settings.acceleration,
            air_acceleration:           settings.acceleration, // TODO
            jump_strength:              settings.jump_strength,
            wall_jump_strength:         settings.wall_jump_strength,
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
            in_control:                 false,
            items_data:                 ItemsData::default(),
        }
    }

    pub fn deal_damage_to(&self, enemy: &mut Enemy) {
        enemy.take_damage(self.damage);
    }

    pub fn take_damage(&mut self, damage: u32) {
        if (self.health as i32) - (damage as i32) >= 0 {
            self.health -= damage;
        } else {
            self.health = 0;
        }
    }

    pub fn gain_reward(&mut self, reward: u32) {
        self.health += reward;
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn is_dead(&self) -> bool {
        self.health <= 0
    }

    pub fn has_extra_jump(&self) -> bool {
        self.items_data.used_extra_jumps < self.items_data.extra_jumps
    }
}

impl Component for Player {
    type Storage = HashMapStorage<Self>;
}

impl Health for Player {
    fn health(&self) -> u32 {
        self.health
    }

    fn health_mut(&mut self) -> &mut u32 {
        &mut self.health
    }

    fn take_damage(&mut self, damage: u32) {
        self.take_damage(damage);
    }
}
