use std::time::{Duration, Instant};

use super::system_prelude::*;
use crate::states::helpers::Timers;

const PRINT_DELAY_MS: u64 = 500;

pub struct TimerSystem {
    last_print_at: Instant,
}

impl<'a> System<'a> for TimerSystem {
    type SystemData = Write<'a, Timers>;

    fn run(&mut self, mut timers: Self::SystemData) {
        let now = Instant::now();
        if now - self.last_print_at >= Duration::from_millis(PRINT_DELAY_MS) {
            timers.level.update().unwrap();
            timers.global.update().unwrap();
            println!("level: {}", timers.level.time_output());
            println!("global: {}", timers.global.time_output());
            self.last_print_at = now;
        }
    }
}

impl Default for TimerSystem {
    fn default() -> Self {
        Self {
            last_print_at: Instant::now(),
        }
    }
}
