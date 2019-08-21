use super::Facing;
use climer::{Time, Timer};
use deathframe::geo::Vector;
use std::time::Duration;

#[derive(Clone)]
pub struct EnemyAiTurretData {
    pub facing:           Facing,
    pub shot_interval_ms: u64,
    pub bullet_velocity:  Vector,
    pub bullet_size:      Vector,
    pub bullet_lifetime:  Duration,
    pub shot_timer:       Timer,
}

impl Default for EnemyAiTurretData {
    fn default() -> Self {
        Self {
            facing:           Default::default(),
            shot_interval_ms: Default::default(),
            bullet_velocity:  Default::default(),
            bullet_size:      Default::default(),
            bullet_lifetime:  Duration::new(5, 0),
            shot_timer:       Timer::builder()
                .time(Time::builder().seconds(4).milliseconds(500).build())
                .quiet(true)
                .build()
                .unwrap(),
        }
    }
}
