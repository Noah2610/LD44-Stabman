use std::time::{Duration, Instant};

use amethyst::utils::fps_counter::FPSCounter;

use super::system_prelude::*;

// 1280 / 720 = 1.777778
// 720 / 1280 = 0.5625

// const CAMERA_RESIZE_STEP: (f32, f32) = (8.0, 8.0);
// const CAMERA_RESIZE_MAX: (f32, f32) = (500.0, 500.0);
// const CAMERA_RESIZE_MIN: (f32, f32) = (-200.0, -200.0);
// const CAMERA_RESIZE_EVERY_MS: u64 = 10;

const fn tup_for_res(res: (f32, f32), num: f32) -> (f32, f32) {
    let scale = res.1 / res.0;
    (num, num * scale)
}

const RES: (f32, f32) = (1280.0, 720.0);
// const CAMERA_RESIZE_STEP: (f32, f32) = tup_for_res(RES, 50.0);
const CAMERA_RESIZE_STEP: (f32, f32) = (50.0, 30.0);
const CAMERA_RESIZE_MAX: (f32, f32) = tup_for_res(RES, 1000.0);
const CAMERA_RESIZE_MIN: (f32, f32) = tup_for_res(RES, -800.0);
const CAMERA_RESIZE_EVERY_MS: u64 = 10;

enum CameraResizeDir {
    Increase,
    Decrease,
}

fn rand_for_camera_resize() -> (f32, f32) {
    use rand::Rng;
    const RANGE: (f32, f32) = (-50.0, 50.0);
    let mut rng = rand::thread_rng();
    (
        rng.gen_range(RANGE.0, RANGE.1),
        rng.gen_range(RANGE.0, RANGE.1),
    )
}

pub struct DebugSystem {
    last_fps_print:     Instant,
    camera_resize_step: (f32, f32),
    camera_resize_dir:  (CameraResizeDir, CameraResizeDir),
    last_camera_resize: Instant,
}

const PRINT_FPS_EVERY_MS: u64 = 1000;

impl<'a> System<'a> for DebugSystem {
    type SystemData = (
        Read<'a, FPSCounter>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, AmethystCamera>,
    );

    fn run(&mut self, (fps_counter, sizes, mut cameras): Self::SystemData) {
        let now = Instant::now();
        if now - self.last_fps_print
            >= Duration::from_millis(PRINT_FPS_EVERY_MS)
        {
            let fps_frame = fps_counter.frame_fps();
            let fps_avg = fps_counter.sampled_fps();
            println!("this_frame: {:.02} average: {:.02}", fps_frame, fps_avg,);
            self.last_fps_print = now;
        }

        // TODO
        {
            use amethyst::renderer::Projection;

            fn resize_camera_step_for(
                dir: &mut CameraResizeDir,
                current: &mut f32,
                step: f32,
                min: f32,
                max: f32,
            ) {
                match dir {
                    CameraResizeDir::Increase => {
                        *current += step;
                        if *current > max {
                            *dir = CameraResizeDir::Decrease;
                        }
                    }
                    CameraResizeDir::Decrease => {
                        *current -= step;
                        if *current < min {
                            *dir = CameraResizeDir::Increase;
                        }
                    }
                }
            }

            if now - self.last_camera_resize
                >= Duration::from_millis(CAMERA_RESIZE_EVERY_MS)
            {
                resize_camera_step_for(
                    &mut self.camera_resize_dir.0,
                    &mut self.camera_resize_step.0,
                    CAMERA_RESIZE_STEP.0,
                    CAMERA_RESIZE_MIN.0,
                    CAMERA_RESIZE_MAX.0,
                );
                resize_camera_step_for(
                    &mut self.camera_resize_dir.1,
                    &mut self.camera_resize_step.1,
                    CAMERA_RESIZE_STEP.1,
                    CAMERA_RESIZE_MIN.1,
                    CAMERA_RESIZE_MAX.1,
                );
                self.last_camera_resize = now;

                for (camera, size) in (&mut cameras, &sizes).join() {
                    // let random1 = rand_for_camera_resize();
                    // let random2 = rand_for_camera_resize();
                    let random1 = (0.0, 0.0);
                    let random2 = (0.0, 0.0);
                    let extra = (
                        self.camera_resize_step.0 * 0.5,
                        self.camera_resize_step.1 * 0.5,
                    );
                    let proj = Projection::orthographic(
                        0.0 - extra.0 + random1.0,    // Left
                        size.w + extra.0 + random2.0, // Right
                        0.0 - extra.1 + random1.1,    // Bottom (!)
                        size.h + extra.1 + random2.1, // Top    (!)
                    );
                    camera.proj = if let Projection::Orthographic(ortho) = proj
                    {
                        ortho.into()
                    } else {
                        panic!("Camera projection has to be orthographic")
                    }
                }
            }
        }
    }
}

impl Default for DebugSystem {
    fn default() -> Self {
        Self {
            last_fps_print:     Instant::now(),
            camera_resize_step: (0.0, 0.0),
            camera_resize_dir:  (
                CameraResizeDir::Increase,
                CameraResizeDir::Increase,
            ),
            last_camera_resize: Instant::now(),
        }
    }
}
