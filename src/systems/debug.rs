use std::time::{Duration, Instant};

use amethyst::utils::fps_counter::FPSCounter;

use super::system_prelude::*;

pub struct DebugSystem {
    last_fps_print: Instant,
}

const PRINT_FPS_EVERY_MS: u64 = 1000;

impl<'a> System<'a> for DebugSystem {
    type SystemData = Read<'a, FPSCounter>;

    fn run(&mut self, fps_counter: Self::SystemData) {
        let now = Instant::now();
        if now - self.last_fps_print
            >= Duration::from_millis(PRINT_FPS_EVERY_MS)
        {
            let fps_frame = fps_counter.frame_fps();
            let fps_avg = fps_counter.sampled_fps();
            println!("this_frame: {:.02} average: {:.02}", fps_frame, fps_avg,);
            self.last_fps_print = now;
        }
    }
}

impl Default for DebugSystem {
    fn default() -> Self {
        Self {
            last_fps_print: Instant::now(),
        }
    }
}
