use super::Facing;
use deathframe::geo::Vector;
use std::time::{Duration, Instant};

#[derive(Clone, PartialEq)]
pub struct EnemyAiTurretData {
    pub facing:           Facing,
    pub shot_interval_ms: u64,
    pub bullet_velocity:  Vector,
    pub bullet_size:      Vector,
    pub bullet_lifetime:  Duration,
    pub last_shot_at:     Instant,
}

impl Default for EnemyAiTurretData {
    fn default() -> Self {
        Self {
            facing:           Default::default(),
            shot_interval_ms: Default::default(),
            bullet_velocity:  Default::default(),
            bullet_size:      Default::default(),
            bullet_lifetime:  Duration::new(5, 0),
            last_shot_at:     Instant::now(),
        }
    }
}
