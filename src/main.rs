extern crate amethyst;
extern crate deathframe;
#[macro_use]
extern crate serde;
extern crate json;

mod components;
mod resource_helpers;
mod settings;
mod states;
mod systems;
mod world_helpers;

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

use resource_helpers::*;
use systems::prelude::*;

fn main() -> amethyst::Result<()> {
    start_logger();

    let game_data = build_game_data()?;

    let mut game: amethyst::CoreApplication<CustomGameData<CustomData>> =
        Application::new("./", states::Startup::default(), game_data)?;
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
        .with("ingame", MoveEntitiesSystem, "move_entities_system", &[
            "gravity_system",
            "limit_velocities_system",
            "player_controls_system",
        ])?
        .with("ingame", CameraSystem, "camera_system", &[
            "move_entities_system",
        ])?
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
            "move_entities_system",
            "collision_system",
            "player_controls_system",
        ])?
        .with(
            "ingame",
            PlayerTakeDamageSystem,
            "player_take_damage_system",
            &[
                "move_entities_system",
                "collision_system",
                "player_controls_system",
            ],
        )?
        .with(
            "ingame",
            HealthDisplaySystem::default(),
            "health_display_system",
            &["player_take_damage_system"],
        )?
        .with("ingame", GoalSystem, "goal_system", &[
            "move_entities_system",
            "collision_system",
        ])?
        .with("ingame", EnemyAiSystem, "enemy_ai_system", &[])?;
    Ok(game_data)
}

#[derive(Clone)]
pub struct CustomData {
    display_config: DisplayConfig,
}
