use std::time::{Duration, Instant};

use amethyst::ui::UiText;

use super::system_prelude::*;
use crate::states::helpers::Timers;

const PRINT_DELAY_MS: u64 = 100;

pub struct TimerSystem {
    last_print_at: Instant,
}

impl<'a> System<'a> for TimerSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        Write<'a, Timers>,
        ReadStorage<'a, TimerUi>,
        WriteStorage<'a, UiText>,
    );

    fn run(
        &mut self,
        (settings, mut timers, timer_uis, mut ui_texts): Self::SystemData,
    ) {
        let now = Instant::now();
        if now - self.last_print_at >= Duration::from_millis(PRINT_DELAY_MS) {
            timers.level.update().unwrap();
            timers.global.as_mut().map(|timer| timer.update().unwrap());

            if settings.level_manager.timers_print_to_stdout {
                self.print_to_stdout(&timers);
            }

            for (timer_ui, ui_text) in (&timer_uis, &mut ui_texts).join() {
                if let Some(time_output) = match timer_ui.timer_type {
                    TimerType::Level => Some(timers.level.time_output()),
                    TimerType::Global => {
                        timers.global.as_ref().map(|timer| timer.time_output())
                    }
                } {
                    ui_text.text =
                        format!("{}{}", timer_ui.text_prefix, time_output);
                }
            }

            self.last_print_at = now;
        }
    }
}

impl TimerSystem {
    fn print_to_stdout(&self, timers: &Timers) {
        println!("level: {}", timers.level.time_output());
        if let Some(global_timer) = timers.global.as_ref() {
            println!("global: {}", global_timer.time_output());
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
