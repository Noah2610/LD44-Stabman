use amethyst::ecs::World;
use amethyst::renderer::{SpriteRender, SpriteSheetHandle};
use deathframe::geo::prelude::*;
use deathframe::handlers::SpriteSheetHandles;
use json::JsonValue;

use crate::components::prelude::*;
use crate::resource_helpers::*;
use crate::settings::SettingsEnemy;
use crate::world_helpers::*;

const ENEMY_NORMAL_SPRITESHEET_FILENAME: &str = "enemy_normal.png";
const ENEMY_CHARGER_SPRITESHEET_FILENAME: &str = "enemy_charger.png";
const ENEMY_FLYING_SPRITESHEET_FILENAME: &str = "enemy_flying.png";
const ENEMY_REAPER_SPRITESHEET_FILENAME: &str = "enemy_reaper.png";
const ENEMY_TURRET_SPRITESHEET_FILENAME: &str = "enemy_turret.png";

pub fn enemy_components_for(
    world: &mut World,
    properties: &JsonValue,
) -> (
    EnemyType,
    SettingsEnemy,
    EnemyAi,
    (SpriteSheetHandle, SpriteRender),
    AnimationsContainer,
    Option<Flipped>,
) {
    {
        let settings = world.settings();
        let mut spritesheet_handles =
            world.write_resource::<SpriteSheetHandles>();

        match properties["enemy_type"].as_str().expect(
            "`enemy_type` property must be given for object of type `Enemy`",
        ) {
            "Normal" => {
                let (spritesheet_handle, sprite_render) = {
                    let handle = spritesheet_handles.get_or_load(
                        resource(format!(
                            "spritesheets/{}",
                            ENEMY_NORMAL_SPRITESHEET_FILENAME
                        )),
                        world,
                    );
                    (handle.clone(), SpriteRender {
                        sprite_sheet:  handle,
                        sprite_number: 0,
                    })
                };
                (
                    EnemyType::Normal,
                    settings.enemies.normal.clone(),
                    EnemyAi::Tracer,
                    (spritesheet_handle.clone(), sprite_render),
                    AnimationsContainer::new()
                        .insert(
                            "idle",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(150)
                                .sprite_ids(vec![0, 1, 2, 3, 4])
                                .build(),
                        )
                        .insert(
                            "walking",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(150)
                                .sprite_ids(vec![2, 3, 4, 3])
                                .build(),
                        )
                        .insert(
                            "hit",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(50)
                                .sprite_ids(vec![3, 4])
                                .build(),
                        )
                        .current("idle")
                        .build(),
                    None,
                )
            }
            "Charger" => {
                let (spritesheet_handle, sprite_render) = {
                    let handle = spritesheet_handles.get_or_load(
                        resource(format!(
                            "spritesheets/{}",
                            ENEMY_CHARGER_SPRITESHEET_FILENAME
                        )),
                        world,
                    );
                    (handle.clone(), SpriteRender {
                        sprite_sheet:  handle,
                        sprite_number: 0,
                    })
                };
                (
                    EnemyType::Charger,
                    settings.enemies.charger.clone(),
                    EnemyAi::Charger(EnemyAiChargerData {
                        stop_moving_when_colliding_sides: Some(vec![
                            Side::Left,
                            Side::Right,
                        ]),
                        ..Default::default()
                    }),
                    (spritesheet_handle.clone(), sprite_render),
                    AnimationsContainer::new()
                        .insert(
                            "idle",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(100)
                                .sprite_ids(vec![0, 1, 0, 7])
                                .build(),
                        )
                        .insert(
                            "walking",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(50)
                                .sprite_ids(vec![0, 1, 2, 3, 4, 5, 6, 7])
                                .build(),
                        )
                        .current("idle")
                        .build(),
                    None,
                )
            }
            "Flying" => {
                let (spritesheet_handle, sprite_render) = {
                    let handle = spritesheet_handles.get_or_load(
                        resource(format!(
                            "spritesheets/{}",
                            ENEMY_FLYING_SPRITESHEET_FILENAME
                        )),
                        world,
                    );
                    (handle.clone(), SpriteRender {
                        sprite_sheet:  handle,
                        sprite_number: 0,
                    })
                };
                (
                    EnemyType::Flying,
                    settings.enemies.flying.clone(),
                    EnemyAi::Tracer,
                    (spritesheet_handle.clone(), sprite_render),
                    AnimationsContainer::new()
                        .insert(
                            "idle",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(100)
                                .sprite_ids(vec![0, 1, 2, 1])
                                .build(),
                        )
                        .insert(
                            "walking",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(100)
                                .sprite_ids(vec![0, 1, 2, 1])
                                .build(),
                        )
                        .current("idle")
                        .build(),
                    None,
                )
            }
            "Reaper" => {
                let (spritesheet_handle, sprite_render) = {
                    let handle = spritesheet_handles.get_or_load(
                        resource(format!(
                            "spritesheets/{}",
                            ENEMY_REAPER_SPRITESHEET_FILENAME
                        )),
                        world,
                    );
                    (handle.clone(), SpriteRender {
                        sprite_sheet:  handle,
                        sprite_number: 0,
                    })
                };
                (
                    EnemyType::Reaper,
                    settings.enemies.reaper.clone(),
                    EnemyAi::Tracer,
                    (spritesheet_handle.clone(), sprite_render),
                    AnimationsContainer::new()
                        .insert(
                            "idle",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(250)
                                .sprite_ids(vec![0, 1])
                                .build(),
                        )
                        .insert(
                            "walking",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(1000)
                                .sprite_ids(vec![2])
                                .build(),
                        )
                        .current("idle")
                        .build(),
                    None,
                )
            }
            "Turret" => {
                // TODO: Flipped
                let (facing, flipped) =
                    if let Some(facing_str) = properties["facing"].as_str() {
                        match facing_str {
                            "Left" => (Facing::Left, Flipped::Horizontal),
                            "Right" => (Facing::Right, Flipped::None),
                            _ => panic!(format!(
                                "Couldn't parse `facing` property for enemy \
                                 `Turret`: {}",
                                facing_str
                            )),
                        }
                    } else {
                        (Facing::default(), Flipped::None)
                    };
                let (spritesheet_handle, sprite_render) = {
                    let handle = spritesheet_handles.get_or_load(
                        resource(format!(
                            "spritesheets/{}",
                            ENEMY_TURRET_SPRITESHEET_FILENAME
                        )),
                        world,
                    );
                    (handle.clone(), SpriteRender {
                        sprite_sheet:  handle,
                        sprite_number: 0,
                    })
                };
                (
                    EnemyType::Turret,
                    settings.enemies.turret.clone(),
                    EnemyAi::Turret(EnemyAiTurretData {
                        facing,
                        shot_interval_ms: settings
                            .enemies
                            .turret_data
                            .shot_interval_ms,
                        bullet_velocity: settings
                            .enemies
                            .turret_data
                            .bullet_velocity,
                        bullet_size: settings.enemies.turret_data.bullet_size,
                        ..Default::default()
                    }),
                    (spritesheet_handle.clone(), sprite_render),
                    AnimationsContainer::new()
                        .insert(
                            "idle",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(250)
                                .sprite_ids(vec![0, 1])
                                .build(),
                        )
                        .insert(
                            "shooting",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(500)
                                .sprite_ids(vec![2, 0, 1])
                                .build(),
                        )
                        .current("idle")
                        .build(),
                    Some(flipped),
                )
            }
            t => panic!(format!("EnemyType '{}' does not exist", t)),
        }
    }
}
