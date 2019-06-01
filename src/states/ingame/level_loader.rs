use std::fs::File;
use std::io::prelude::*;

use amethyst::ecs::world::Index;
use deathframe::geo::{Anchor, Rect, Side, Vector};
use json::JsonValue;

use super::super::state_prelude::*;
use crate::components::prelude::*;
use crate::settings::SettingsLevelManager;
use crate::solid_tag::SolidTag;

const PROPERTY_Z_KEY: &str = "z";
const PLAYER_Z: f32 = 0.5;
const CAMERA_Z: f32 = 10.0;
const TILE_Z: f32 = 0.0;
const PARALLAX_Z: f32 = -1.0;
const ENEMY_Z: f32 = 0.25;
const GOAL_Z: f32 = 0.1;
const ITEM_Z: f32 = 0.6;
const PLAYER_SPRITESHEET_FILENAME: &str = "player.png";
const BACKGROUNDS_DIR: &str = "textures/bg";
const ENEMY_NORMAL_SPRITESHEET_FILENAME: &str = "enemy_normal.png";
const ENEMY_CHARGER_SPRITESHEET_FILENAME: &str = "enemy_charger.png";
const ENEMY_FLYING_SPRITESHEET_FILENAME: &str = "enemy_flying.png";
const ENEMY_REAPER_SPRITESHEET_FILENAME: &str = "enemy_reaper.png";
const ENEMY_TURRET_SPRITESHEET_FILENAME: &str = "enemy_reaper.png"; // TODO

struct SpriteData {
    pub spritesheet_path: String,
    pub sprite_id:        usize,
}

struct TextureData {}

enum Graphic {
    Sprite(SpriteData),
    Texture(TextureData),
}

struct EntityData {
    pub pos:        Vector,
    pub size:       Vector,
    pub properties: JsonValue,
    pub graphic:    Option<Graphic>,
}

pub struct LevelLoader {
    settings:      SettingsLevelManager,
    level_size:    Option<Vector>,
    camera_id:     Option<Index>,
    player_id:     Option<Index>,
    player_data:   Option<EntityData>,
    tiles_data:    Vec<EntityData>,
    parallax_data: Vec<EntityData>,
    enemies_data:  Vec<EntityData>,
    goal_data:     Option<EntityData>,
    items_data:    Vec<EntityData>,
}

impl LevelLoader {
    pub fn new(settings: SettingsLevelManager) -> Self {
        Self {
            settings:      settings,
            level_size:    None,
            camera_id:     None,
            player_id:     None,
            player_data:   None,
            tiles_data:    Vec::new(),
            parallax_data: Vec::new(),
            enemies_data:  Vec::new(),
            goal_data:     None,
            items_data:    Vec::new(),
        }
    }

    /// Returns `true` if everything has finished loading and building properly.
    pub fn is_finished(&self) -> bool {
        self.player_id.is_some() && self.camera_id.is_some()
    }

    /// Start loading the level data from the given level filename.
    pub fn load_level<T>(&mut self, filepath: T)
    where
        T: ToString,
    {
        let filepath = filepath.to_string();
        let mut file = File::open(&filepath)
            .expect(&format!("Should open file for reading: {}", filepath));
        let mut json_raw = String::new();
        file.read_to_string(&mut json_raw)
            .expect(&format!("Should read file content: {}", filepath));
        let json = json::parse(&json_raw)
            .expect(&format!("Could not parse JSON for level: {}", filepath));

        self.load_level_data(&json["level"]);
        self.load_objects(&json["objects"]);
        self.load_tiles(&json["tiles"]);
    }

    /// Builds the loaded data using the given `StateData`.
    pub fn build<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        self.build_player(data);
        self.build_camera(data);
        self.build_tiles(data);
        self.build_parallax(data);
        self.build_enemies(data);
        self.build_goal(data);
        self.build_items(data);
    }

    fn load_level_data(&mut self, json: &JsonValue) {
        const ERRMSG: &str = "\"level\".\"size\" values should be f32";
        for (key, val) in json.entries() {
            match key {
                "size" => {
                    self.level_size = Some(Vector::new(
                        val["w"].as_f32().expect(ERRMSG),
                        val["h"].as_f32().expect(ERRMSG),
                    ))
                }
                _ => (),
            }
        }
    }

    fn load_objects(&mut self, json: &JsonValue) {
        for object_data in json.members() {
            if let (
                Some(obj_type),
                (Some(x), Some(y)),
                (Some(w), Some(h)),
                properties,
            ) = (
                object_data["type"].as_str(),
                (
                    object_data["pos"]["x"].as_f32(),
                    object_data["pos"]["y"].as_f32(),
                ),
                (
                    object_data["size"]["w"].as_f32(),
                    object_data["size"]["h"].as_f32(),
                ),
                &object_data["properties"],
            ) {
                let pos = (x, y).into();
                let size = (w, h).into();
                match obj_type {
                    "Player" => {
                        self.player_data = Some(EntityData {
                            pos:        pos,
                            size:       size,
                            properties: properties.clone(),
                            graphic:    None,
                        })
                    }
                    "Parallax" => self.parallax_data.push(EntityData {
                        pos:        pos,
                        size:       size,
                        properties: properties.clone(),
                        graphic:    None,
                    }),
                    "Enemy" => self.enemies_data.push(EntityData {
                        pos:        pos,
                        size:       size,
                        properties: properties.clone(),
                        graphic:    None,
                    }),
                    "Goal" => {
                        self.goal_data = Some(EntityData {
                            pos:        pos,
                            size:       size,
                            properties: properties.clone(),
                            graphic:    None,
                        })
                    }
                    "Item" => self.items_data.push(EntityData {
                        pos:        pos,
                        size:       size,
                        properties: properties.clone(),
                        graphic:    None,
                    }),
                    _ => (),
                }
            }
        }
    }

    fn load_tiles(&mut self, json: &JsonValue) {
        for tile_data in json.members() {
            if let (
                Some(id),
                (Some(x), Some(y)),
                properties,
                Some(tileset_name),
            ) = (
                tile_data["id"].as_usize(),
                (
                    tile_data["pos"]["x"].as_f32(),
                    tile_data["pos"]["y"].as_f32(),
                ),
                &tile_data["properties"],
                tile_data["ts"].as_str(),
            ) {
                let spritesheet_path =
                    resource(format!("spritesheets/{}.png", tileset_name));

                self.tiles_data.push(EntityData {
                    pos:        (x, y).into(),
                    size:       self.settings.tile_size,
                    properties: properties.clone(),
                    graphic:    Some(Graphic::Sprite(SpriteData {
                        spritesheet_path: spritesheet_path,
                        sprite_id:        id,
                    })),
                });
            }
        }
    }

    fn build_player<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        if let Some(EntityData {
            pos,
            size,
            properties,
            graphic: _,
        }) = &self.player_data
        {
            let settings = data.world.settings();

            let mut transform = Transform::default();
            transform.set_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(PLAYER_Z),
            ); // NOTE: Draw player above foreground elements
            let size = Size::from(*size);

            let spritesheet_path = resource(format!(
                "spritesheets/{}",
                PLAYER_SPRITESHEET_FILENAME
            ));
            let (spritesheet_handle, sprite_render, atk_sprite_render) = {
                let spritesheet_handle = data
                    .world
                    .write_resource::<SpriteSheetHandles>()
                    .get_or_load(spritesheet_path, &data.world);
                (
                    spritesheet_handle.clone(),
                    SpriteRender {
                        sprite_sheet:  spritesheet_handle.clone(),
                        sprite_number: 0,
                    },
                    SpriteRender {
                        sprite_sheet:  spritesheet_handle,
                        sprite_number: 0, // TODO: Initialize with proper ID
                    },
                )
            };

            let player = data
                .world
                .create_entity()
                .with(Player::new(settings.player.clone()))
                .with(transform.clone())
                .with(sprite_render)
                .with(Transparent)
                .with(Velocity::default())
                // .with(MaxVelocity::from(settings.player.max_velocity)) // TODO
                .with(DecreaseVelocity::from(settings.player.decr_velocity))
                .with(size.clone())
                .with(ScaleOnce)
                .with(Gravity::from(settings.player.gravity))
                .with(Solid::new(SolidTag::Player))
                .with(Collision::new())
                .with(CheckCollision)
                .with(Push)
                .with(
                    AnimationsContainer::new()
                        .insert(
                            "idle",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(500)
                                .sprite_ids(vec![0, 1])
                                .build(),
                        )
                        .insert(
                            "walking",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(100)
                                .sprite_ids(vec![2, 3, 4, 5, 6, 7])
                                .build(),
                        )
                        .insert(
                            "falling",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(1000)
                                .sprite_ids(vec![8])
                                .build(),
                        )
                        .insert(
                            "attack",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .delays_ms(vec![50, 50, 100, 75, 100])
                                .sprite_ids(vec![11, 9, 13, 9, 11])
                                .build(),
                        )
                        // TODO: Un-nused
                        .insert(
                            "level_start",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(500)
                                .sprite_ids(vec![0, 2, 10, 5])
                                .build(),
                        )
                        .insert(
                            "level_end",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(500)
                                .sprite_ids(vec![5, 10, 2, 12])
                                .build(),
                        )
                        .insert(
                            "death",
                            Animation::new()
                                .default_sprite_sheet_handle(
                                    spritesheet_handle.clone(),
                                )
                                .default_delay_ms(500)
                                .sprite_ids(vec![5, 10, 2, 12])
                                .build(),
                        )
                        .current("idle")
                        .build(),
                )
                .with(Flipped::None)
                .build();
            self.player_id = Some(player.id());

            // Create PlayerAttack entity
            data.world
                .create_entity()
                .with(PlayerAttack::default())
                .with(transform)
                .with(atk_sprite_render)
                .with(Transparent)
                .with(size)
                .with(ScaleOnce)
                .with(Collision::new())
                .with(CheckCollision)
                .with(
                    AnimationsContainer::new()
                        .insert(
                            "attack_default",
                            Animation::new()
                                .default_sprite_sheet_handle(spritesheet_handle)
                                .delays_ms(vec![50, 50, 100, 75, 100])
                                .sprite_ids(vec![12, 10, 14, 10, 12])
                                .build(),
                        )
                        .build(),
                )
                .with(Flipped::None)
                .with(Hidden)
                .build();
        }
    }

    fn build_camera<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        let settings = data.world.settings();

        let mut transform = Transform::default();
        transform.set_z(CAMERA_Z);

        let mut camera = Camera::new()
            .base_speed({ settings.camera.base_speed })
            .deadzone({ settings.camera.deadzone });

        if let Some(player_id) = self.player_id {
            camera = camera.follow(player_id);
        }

        let mut entity_builder = data
            .world
            .create_entity()
            .with(AmethystCamera::from(Projection::orthographic(
                0.0,                    // Left
                settings.camera.size.0, // Right
                0.0,                    // Bottom (!)
                settings.camera.size.1, // Top    (!)
            )))
            .with(camera.build())
            .with(transform)
            .with(Size::from(settings.camera.size))
            .with(InnerSize(Size::from(settings.camera.inner_size)))
            .with(Velocity::default())
            .with(Collision::new());

        if let Some(size) = self.level_size {
            // NOTE: Offset the values by half of camera's size,
            // because the `ConfineEntitiesSystem` assumes the entity's
            // anchor point is in the center. The Camera is the only
            // entity for which the anchor point is in the bottom left.
            entity_builder = entity_builder.with(Confined::new(Rect {
                top:    size.1 - settings.camera.size.1 * 0.5,
                bottom: 0.0 - settings.camera.size.1 * 0.5,
                left:   0.0 - settings.camera.size.0 * 0.5,
                right:  size.0 - settings.camera.size.0 * 0.5,
            }));
        }

        let entity = entity_builder.build();

        self.camera_id = Some(entity.id());
    }

    fn build_tiles<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        for EntityData {
            pos,
            size,
            properties,
            graphic,
        } in &self.tiles_data
        {
            let mut transform = Transform::default();
            transform.set_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(TILE_Z),
            );

            let sprite_render_opt =
                if let Some(Graphic::Sprite(sprite_data)) = graphic {
                    let sprite_render = {
                        let spritesheet_handle = data
                            .world
                            .write_resource::<SpriteSheetHandles>()
                            .get_or_load(
                                &sprite_data.spritesheet_path,
                                &data.world,
                            );
                        SpriteRender {
                            sprite_sheet:  spritesheet_handle,
                            sprite_number: sprite_data.sprite_id,
                        }
                    };
                    Some(sprite_render)
                } else {
                    None
                };

            let mut entity = data
                .world
                .create_entity()
                .with(transform)
                .with(Size::from(*size))
                .with(ScaleOnce)
                .with(Transparent);

            if let Some(sprite_render) = sprite_render_opt {
                entity = entity.with(sprite_render);
            }

            for component_name in properties["components"].members() {
                let component_name_str = component_name
                    .as_str()
                    .expect("Could not parse string JSON");
                entity =
                    deathframe::components::add_component_to_entity_by_name(
                        entity,
                        component_name_str,
                    );
                // TODO
                // entity =
                //     crate::components::add_component_to_entity_by_name_custom(
                //         entity,
                //         component_name_str,
                //     );
            }

            if let Some(is_solid) = properties["solid"].as_bool() {
                if is_solid {
                    entity = entity
                        .with(Solid::new(SolidTag::default()))
                        .with(Collision::new());
                }
            }

            entity.build();
        }
    }

    fn build_parallax<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        let bg_dir = resource(BACKGROUNDS_DIR);

        for EntityData {
            pos,
            size,
            properties,
            graphic: _,
        } in &self.parallax_data
        {
            if let Some(camera_id) = self.camera_id {
                // Load bg image texture
                let texture_handle_opt = if let Some((_, bg_filename)) =
                    properties.entries().find(|(key, _)| key == &"image")
                {
                    let mut texture_handles =
                        data.world.write_resource::<TextureHandles>();
                    let filepath = format!(
                        "{}/{}",
                        bg_dir,
                        bg_filename.as_str().expect(
                            "Couldn't parse background image filename as str"
                        )
                    );
                    Some(texture_handles.get_or_load(filepath, data.world))
                } else {
                    None
                };

                // Create entity
                let mut entity = data.world.create_entity();
                let mut parallax = Parallax::new()
                    .follow(camera_id)
                    .follow_anchor(Anchor::BottomLeft);

                let mut has_set_offset = false;

                for (key, val) in properties.entries() {
                    match (key, &texture_handle_opt) {
                        ("speed_mult", _) => {
                            parallax = parallax.speed_mult(
                                parse_string_to_vector(val.as_str().expect(
                                    "Couldn't parse JsonValue as string",
                                )),
                            );
                        }
                        ("speed_mult_x", _) => {
                            parallax = parallax.speed_mult_x(
                                val.as_f32()
                                    .expect("Couldn't parse JsonValue as f32"),
                            );
                        }
                        ("speed_mult_y", _) => {
                            parallax = parallax.speed_mult_y(
                                val.as_f32()
                                    .expect("Couldn't parse JsonValue as f32"),
                            );
                        }
                        ("offset", _) => {
                            has_set_offset = true;
                            parallax = parallax.offset(parse_string_to_vector(
                                val.as_str().expect(
                                    "Couldn't parse JsonValue as string",
                                ),
                            ));
                        }
                        ("offset_x", _) => {
                            parallax = parallax.offset_x(
                                val.as_f32()
                                    .expect("Couldn't parse JsonValue as f32"),
                            );
                        }
                        ("offset_y", _) => {
                            parallax = parallax.offset_y(
                                val.as_f32()
                                    .expect("Couldn't parse JsonValue as f32"),
                            );
                        }
                        ("image", Some(texture_handle)) => {
                            entity = entity.with(texture_handle.clone());
                        }
                        _ => (),
                    }
                }

                // Set offset as parallax object position, unless 'offset' property was given
                if !has_set_offset {
                    parallax = parallax.offset(*pos);
                }

                // Add transform and size to entity
                let mut transform = Transform::default();
                transform.set_xyz(
                    pos.0,
                    pos.1,
                    properties[PROPERTY_Z_KEY].as_f32().unwrap_or(PARALLAX_Z),
                ); // NOTE: Draw parallax backgrounds behind foreground
                entity = entity
                    .with(transform)
                    .with(Size::from(*size))
                    .with(Velocity::default())
                    .with(ScaleOnce)
                    .with(Transparent)
                    .with(parallax.build());

                entity.build();
            }
        }
    }

    fn build_enemies<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        let settings = data.world.settings();

        for EntityData {
            pos,
            size,
            properties,
            graphic: _,
        } in &self.enemies_data
        {
            let (
                enemy_type,
                enemy_settings,
                enemy_ai,
                (spritesheet_handle, sprite_render),
                animations_container,
            ) = {
                let mut spritesheet_handles =
                    data.world.write_resource::<SpriteSheetHandles>();

                match properties["enemy_type"].as_str().expect(
                    "`enemy_type` property must be given for object of type \
                     `Enemy`",
                ) {
                    "Normal" => {
                        let (spritesheet_handle, sprite_render) = {
                            let handle = spritesheet_handles.get_or_load(
                                resource(format!(
                                    "spritesheets/{}",
                                    ENEMY_NORMAL_SPRITESHEET_FILENAME
                                )),
                                data.world,
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
                        )
                    }
                    "Charger" => {
                        let (spritesheet_handle, sprite_render) = {
                            let handle = spritesheet_handles.get_or_load(
                                resource(format!(
                                    "spritesheets/{}",
                                    ENEMY_CHARGER_SPRITESHEET_FILENAME
                                )),
                                data.world,
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
                                        .sprite_ids(vec![
                                            0, 1, 2, 3, 4, 5, 6, 7,
                                        ])
                                        .build(),
                                )
                                .current("idle")
                                .build(),
                        )
                    }
                    "Flying" => {
                        let (spritesheet_handle, sprite_render) = {
                            let handle = spritesheet_handles.get_or_load(
                                resource(format!(
                                    "spritesheets/{}",
                                    ENEMY_FLYING_SPRITESHEET_FILENAME
                                )),
                                data.world,
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
                        )
                    }
                    "Reaper" => {
                        let (spritesheet_handle, sprite_render) = {
                            let handle = spritesheet_handles.get_or_load(
                                resource(format!(
                                    "spritesheets/{}",
                                    ENEMY_REAPER_SPRITESHEET_FILENAME
                                )),
                                data.world,
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
                        )
                    }
                    "Turret" => {
                        // TODO: Flipped
                        let facing = if let Some(facing_str) =
                            properties["facing"].as_str()
                        {
                            match facing_str {
                                "Left" => Facing::Left,
                                "Right" => Facing::Right,
                                _ => panic!(format!(
                                    "Couldn't parse `facing` property for \
                                     enemy `Turret`: {}",
                                    facing_str
                                ),),
                            }
                        } else {
                            Facing::default()
                        };
                        let (spritesheet_handle, sprite_render) = {
                            let handle = spritesheet_handles.get_or_load(
                                resource(format!(
                                    "spritesheets/{}",
                                    ENEMY_TURRET_SPRITESHEET_FILENAME
                                )),
                                data.world,
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
                                bullet_size: settings
                                    .enemies
                                    .turret_data
                                    .bullet_size,
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
                        )
                    }
                    t => panic!(format!("EnemyType '{}' does not exist", t)),
                }
            };

            let mut transform = Transform::default();
            transform.set_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(ENEMY_Z),
            );

            let mut entity = data
                .world
                .create_entity()
                .with(transform)
                .with(Size::from(*size))
                .with(Velocity::default())
                // .with(MaxVelocity::from(enemy_settings.max_velocity)) // TODO
                .with(DecreaseVelocity::from(enemy_settings.decr_velocity))
                .with(Collision::new())
                .with(CheckCollision)
                .with(Solid::new(SolidTag::Enemy))
                .with(ScaleOnce)
                .with(Enemy::new(enemy_type.clone(), enemy_settings))
                .with(sprite_render)
                .with(Flipped::None)
                .with(animations_container)
                .with(Transparent)
                .with(enemy_ai);

            if enemy_type != EnemyType::Flying
                && enemy_type != EnemyType::Turret
            {
                entity = entity.with(Gravity::from(settings.enemies.gravity));
            }

            entity.build();
        }
    }

    fn build_goal<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        if let Some(EntityData {
            pos,
            size,
            properties,
            graphic: _,
        }) = &self.goal_data
        {
            let mut transform = Transform::default();
            transform.set_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(GOAL_Z),
            );

            data.world
                .create_entity()
                .with(Goal::default())
                .with(transform)
                .with(Size::from(*size))
                .with(Collision::new())
                .build();
        }
    }

    fn build_items<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        let settings = data.world.settings();

        for EntityData {
            pos,
            size,
            properties,
            graphic: _,
        } in &self.items_data
        {
            let (item, spritesheet_handle, sprite_render) = {
                let item = Item::new(
                    properties["item_type"].as_str().expect(
                        "`item_type` property must be given for object of \
                         type `Item`",
                    ),
                    &settings.items,
                );
                let (spritesheet_handle, sprite_render) = item
                    .item_type
                    .sprite_sheet_handle_and_sprite_render(&mut data.world);
                (item, spritesheet_handle, sprite_render)
            };

            let mut transform = Transform::default();
            transform.set_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(ITEM_Z),
            );

            data.world
                .create_entity()
                .with(item)
                .with(transform)
                .with(Size::from(*size))
                .with(ScaleOnce)
                .with(Collision::new())
                .with(sprite_render)
                .with(Transparent)
                .build();
        }
    }
}

fn parse_string_to_vector<T>(string: T) -> Vector
where
    T: ToString,
{
    let string = string.to_string();
    let vec = string
        .split(",")
        .map(|s| {
            s.trim()
                .parse::<f32>()
                .expect(&format!("Couldn't parse string as f32: '{:?}'", s))
        })
        .collect::<Vec<f32>>();
    if vec.len() == 2 {
        (vec[0], vec[1]).into()
    } else {
        panic!(format!(
            "Given string does not have exactly two fields for Vector (x, y): \
             '{:?}'",
            string
        ));
    }
}
