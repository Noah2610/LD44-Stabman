extern crate amethyst;
extern crate deathframe;
#[macro_use]
extern crate serde;
extern crate json;

mod bullet_creator;
mod components;
mod resource_helpers;
mod settings;
mod solid_tag;
mod states;
mod systems;
mod world_helpers;

use std::time::Duration;

use amethyst::audio::AudioBundle;
use amethyst::core::frame_limiter::FrameRateLimitStrategy;
use amethyst::core::transform::TransformBundle;
use amethyst::input::InputBundle;
use amethyst::prelude::*;
use amethyst::renderer::{
    ColorMask,
    DepthMode,
    DisplayConfig,
    DrawFlat2D,
    Pipeline,
    RenderBundle,
    Stage,
    ALPHA,
};
use amethyst::ui::{DrawUi, UiBundle};
use amethyst::utils::fps_counter::FPSCounterBundle;
use amethyst::{LogLevelFilter, LoggerConfig};

use deathframe::custom_game_data::prelude::*;
use deathframe::handlers::AudioHandles;

use resource_helpers::*;
use systems::prelude::*;

const FPS: u32 = 60;
const SLEEP_AND_YIELD_MS: u64 = 2;

fn main() -> amethyst::Result<()> {
    start_logger();

    let game_data = build_game_data()?;

    let mut game: amethyst::CoreApplication<CustomGameData<CustomData>> =
        Application::build("./", states::Startup::default())?
            // https://docs-src.amethyst.rs/stable/amethyst_core/frame_limiter
            .with_frame_limit(
                FrameRateLimitStrategy::SleepAndYield(Duration::from_millis(
                    SLEEP_AND_YIELD_MS,
                )),
                FPS,
            )
            .build(game_data)?;
    game.run();

    Ok(())
}

fn start_logger() {
    amethyst::start_logger(LoggerConfig {
        level_filter: LogLevelFilter::Error,
        ..Default::default()
    });
}

fn build_game_data<'a, 'b>(
) -> amethyst::Result<CustomGameDataBuilder<'a, 'b, CustomData>> {
    // Display config
    let display_config = DisplayConfig::load(&resource("config/display.ron"));

    // CustomGameData CustomData
    let custom_data = CustomData {
        display_config: display_config.clone(),
    };

    // Pipeline
    let pipeline = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.2, 0.2, 0.2, 1.0], 10.0)
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                // NOTE: I have no idea what this `DepthMode` does, as it isn't documented,
                //       but sprite ordering via their z positions only works with this `DepthMode` variant.
                Some(DepthMode::LessEqualWrite),
            ))
            .with_pass(DrawUi::new()), // NOTE: "It's recommended this be your last pass."
    );

    // Bundles
    let transform_bundle = TransformBundle::new();
    let render_bundle =
        RenderBundle::new(pipeline, Some(display_config.clone()))
            .with_sprite_sheet_processor()
            .with_sprite_visibility_sorting(&["transform_system"]);
    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(&resource("config/bindings.ron"))?;
    let ui_bundle = UiBundle::<String, String>::new();
    let audio_bundle = AudioBundle::new(|_: &mut AudioHandles| None); // I hate this
    let fps_bundle = FPSCounterBundle;

    // Create GameDataBuilder
    let game_data = CustomGameData::<CustomData>::new()
        .custom(custom_data)
        .dispatcher("startup")?
        .dispatcher("main_menu")?
        .dispatcher("ingame")?
        .dispatcher("paused")?
        .with_core_bundle(transform_bundle)?
        .with_core_bundle(render_bundle)?
        .with_core_bundle(input_bundle)?
        .with_core_bundle(ui_bundle)?
        .with_core_bundle(fps_bundle)?
        .with_bundle("ingame", audio_bundle)?
        .with_core(InputManagerSystem, "input_manager_system", &[
            "input_system",
        ])?
        .with_core(ScaleSpritesSystem, "scale_sprites_system", &[])?
        .with_core(DebugSystem::default(), "debug_system", &[])?
        .with("ingame", PlayerControlsSystem, "player_controls_system", &[
        ])?
        .with("ingame", GravitySystem, "gravity_system", &[])?
        .with(
            "ingame",
            LimitVelocitiesSystem,
            "limit_velocities_system",
            &["gravity_system", "player_controls_system"],
        )?
        .with(
            "ingame",
            MoveEntitiesSystem::<solid_tag::SolidTag>::default(),
            "move_entities_system",
            &[
                "gravity_system",
                "limit_velocities_system",
                "player_controls_system",
            ],
        )?
        .with("ingame", CameraSystem, "camera_system", &[
            "move_entities_system",
        ])?
        .with(
            "ingame",
            ConfineEntitiesSystem,
            "confine_entities_system",
            &["move_entities_system", "camera_system"],
        )?
        .with("ingame", ParallaxSystem, "parallax_system", &[
            "move_entities_system",
            "camera_system",
        ])?
        .with("ingame", CollisionSystem, "collision_system", &[
            "move_entities_system",
        ])?
        .with(
            "ingame",
            DecreaseVelocitiesSystem,
            "decrease_velocities_system",
            &[
                "gravity_system",
                "limit_velocities_system",
                "move_entities_system",
                "player_controls_system",
            ],
        )?
        .with("ingame", AnimationSystem, "animation_system", &[])?
        .with("ingame", PlayerAttackSystem, "player_attack_system", &[
            "player_controls_system",
            "decrease_velocities_system",
            "limit_velocities_system",
            "collision_system",
        ])?
        .with(
            "ingame",
            PlayerTakeDamageSystem,
            "player_take_damage_system",
            &["player_controls_system", "collision_system"],
        )?
        .with(
            "ingame",
            HealthDisplaySystem::default(),
            "health_display_system",
            &["player_take_damage_system"],
        )?
        .with("ingame", GoalSystem, "goal_system", &["collision_system"])?
        .with("ingame", BulletSystem, "bullet_system", &[
            "collision_system",
        ])?
        .with(
            "ingame",
            SyncHeartsContainersWithHealthSystem,
            "sync_hearts_containers_with_health",
            &[
                "collision_system",
                "player_attack_system",
                "player_take_damage_system",
                "bullet_system",
            ],
        )?
        .with("ingame", EnemyAiSystem, "enemy_ai_system", &[
            "decrease_velocities_system",
            "limit_velocities_system",
            "player_attack_system",
            "collision_system",
            "sync_hearts_containers_with_health",
        ])?
        .with("ingame", BulletCreatorSystem, "bullet_creator_system", &[
            "player_controls_system",
            "enemy_ai_system",
        ])?
        .with("ingame", HarmfulSystem, "harmful_system", &[
            "collision_system",
        ])?
        .with(
            "ingame",
            PlayerDashSystem::default(),
            "player_dash_system",
            &["move_entities_system"],
        )?
        .with("ingame", LoaderSystem, "loader_system", &[
            "move_entities_system",
        ])?
        .with("ingame", HeartsSystem::default(), "hearts_system", &[
            "move_entities_system",
            "player_attack_system",
        ])?;
    Ok(game_data)
}

#[derive(Clone)]
pub struct CustomData {
    display_config: DisplayConfig,
}
