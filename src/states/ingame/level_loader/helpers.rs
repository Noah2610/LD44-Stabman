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

pub fn enemy_components_from(
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

/// Generate an Animation from the given properties.
pub fn animation_from(
    spritesheet_handle: SpriteSheetHandle,
    properties: &JsonValue,
) -> Option<Animation> {
    match (
        properties["animation_sprite_ids"].as_str(),
        properties["animation_delays_ms"].as_str(),
    ) {
        (Some(str_sprite_ids), Some(str_delays_ms)) => {
            let sprite_ids = str_sprite_ids
                .split(",")
                .map(|str_id| {
                    str_id.trim().parse::<usize>().expect(&format!(
                        "Couldn't parse string to usize '{}' in '{}' \
                         (animation_sprite_ids)",
                        str_id, str_sprite_ids
                    ))
                })
                .collect();
            let delays_ms = str_delays_ms
                .split(",")
                .map(|str_ms| {
                    str_ms.trim().parse::<u64>().expect(&format!(
                        "Couldn't parse string to u64 '{}' in '{}' \
                         (animation_delays_ms)",
                        str_ms, str_delays_ms
                    ))
                })
                .collect();
            Some(
                Animation::new()
                    .default_sprite_sheet_handle(spritesheet_handle)
                    .sprite_ids(sprite_ids)
                    .delays_ms(delays_ms)
                    .build(),
            )
        }
        (Some(_), None) | (None, Some(_)) => panic!(
            "Tile with animation needs both properties `animation_sprite_ids` \
             and `animation_delays_ms`"
        ),
        (None, None) => None,
    }
}

/// Generate a AnimationsContainer from the the given animations ron file.
pub fn animations_container_from_file<T>(
    file: T,
    spritesheet_handle: SpriteSheetHandle,
) -> AnimationsContainer
where
    T: ToString,
{
    let animations_container_config = load_animations_container_config(file);
    let mut animations_container = AnimationsContainer::new();

    for animation_config in animations_container_config.animations {
        let mut animation = Animation::new()
            .default_sprite_sheet_handle(spritesheet_handle.clone());
        if let Some(default_delay_ms) = animation_config.default_delay_ms {
            animation = animation.default_delay_ms(default_delay_ms);
        }
        if let Some(delays_ms) = animation_config.delays_ms {
            animation = animation.delays_ms(delays_ms);
        }
        animation = animation.sprite_ids(animation_config.sprite_ids);

        animations_container = animations_container
            .insert(animation_config.name, animation.build());
    }

    if let Some(current) = animations_container_config.current {
        animations_container = animations_container.current(current);
    }

    animations_container.build()
}

#[derive(Deserialize)]
struct AnimationConfig {
    pub name:             String,
    pub sprite_ids:       Vec<usize>,
    pub delays_ms:        Option<Vec<u64>>,
    pub default_delay_ms: Option<u64>,
}

#[derive(Deserialize)]
struct AnimationsContainerConfig {
    pub animations: Vec<AnimationConfig>,
    pub current:    Option<String>,
}

fn load_animations_container_config<T>(file: T) -> AnimationsContainerConfig
where
    T: ToString,
{
    let settings_raw = read_file(file.to_string())
        .expect(&format!("Couldn't read file {}", file.to_string()));
    ron::Value::from_str(&settings_raw)
        .unwrap()
        .into_rust()
        .unwrap()
}
