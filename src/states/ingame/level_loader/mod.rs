mod helpers;

use std::fs::File;
use std::io::prelude::*;

use amethyst::ecs::world::Index;
use deathframe::geo::{Anchor, Rect, Side, Vector};
use json::JsonValue;

use super::super::state_prelude::*;
use crate::components::prelude::*;
use crate::settings::SettingsLevelManager;
use crate::solid_tag::SolidTag;
use helpers::*;

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
    pub fn build(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
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
                let size = Vector::new(w, h);
                let pos = Vector::new(x + size.0 * 0.5, y - size.1 * 0.5);

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

                let size = self.settings.tile_size;
                let pos = Vector::new(x + size.0 * 0.5, y - size.1 * 0.5);

                self.tiles_data.push(EntityData {
                    pos,
                    size,
                    properties: properties.clone(),
                    graphic: Some(Graphic::Sprite(SpriteData {
                        spritesheet_path: spritesheet_path,
                        sprite_id:        id,
                    })),
                });
            }
        }
    }

    fn build_player(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
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

            let mut player_id = 0;
            {
                // TODO
                use rand::Rng;
                let mut rng = rand::thread_rng();

                for i in 1 .. 100 {
                    let mut transform = transform.clone();
                    let pos = transform.translation();
                    transform.set_xyz(
                        pos.x + 16.0 * i as f32,
                        pos.y + 128.0,
                        pos.z - 0.01,
                    );
                    let size = Size::new(
                        rng.gen_range(4.0, 96.0),
                        rng.gen_range(4.0, 192.0),
                    );

                    let mut settings = settings.player.clone();
                    settings.max_velocity.0 = Some(rng.gen_range(
                        50.0,
                        settings.max_velocity.0.unwrap_or(500.0) * 4.0,
                    ));
                    settings.acceleration.0 =
                        rng.gen_range(10.0, settings.acceleration.0 * 8.0);
                    settings.jump_strength =
                        rng.gen_range(100.0, settings.jump_strength * 6.0);

                    settings.gravity = Vector::new(
                        rng.gen_range(-10.0, 10.0),
                        rng.gen_range(settings.gravity.1 * 8.0, 10.0),
                    );
                    settings.jump_gravity = Vector::new(
                        rng.gen_range(settings.jump_gravity.0 * 4.0, 10.0),
                        rng.gen_range(-100.0, 100.0),
                    );
                    settings.slide_strength =
                        rng.gen_range(2.0, settings.slide_strength * 4.0);

                    let mut player = Player::new(settings.clone());
                    player.items_data.extra_jump.extra_jumps = 10;
                    player.items_data.wall_jump.can_wall_jump = true;
                    player.items_data.knockback.has_knockback = true;
                    player.items_data.knockback.velocity = (200.0, 75.0).into();
                    player.items_data.bullet_shoot.can_shoot = true;
                    player.items_data.bullet_shoot.damage = 1;
                    player.items_data.bullet_shoot.velocity = (
                        rng.gen_range(10.0, 500.0 * 4.0),
                        rng.gen_range(-500.0, 500.0),
                    )
                        .into();
                    player.items_data.bullet_shoot.size = (
                        rng.gen_range(4.0, 16.0 * 8.0),
                        rng.gen_range(4.0, 16.0 * 8.0),
                    )
                        .into();
                    player.items_data.bullet_shoot.lifetime =
                        std::time::Duration::from_millis(2500);
                    player.items_data.dash.dashes = 10;
                    player.items_data.dash.duration_ms =
                        rng.gen_range(50, 150 * 4);
                    player.items_data.dash.velocity = (
                        rng.gen_range(10.0, 550.0 * 4.0),
                        rng.gen_range(10.0, 250.0 * 4.0),
                    )
                        .into();
                    player.items_data.thrust.can_thrust = true;
                    player.items_data.thrust.strength = (
                        rng.gen_range(-400.0, 400.0 * 4.0),
                        rng.gen_range(-200.0, 200.0 * 4.0),
                    )
                        .into();

                    player_id = data
                        .world
                        .create_entity()
                        .with(player)
                        .with(transform)
                        .with(sprite_render.clone())
                        .with(Transparent)
                        .with(Velocity::default())
                        // .with(MaxVelocity::from(settings.player.max_velocity)) // TODO
                        .with(DecreaseVelocity::from(settings.decr_velocity))
                        .with(size)
                        .with(ScaleOnce)
                        .with(Gravity::from(settings.gravity))
                        .with(Solid::new(SolidTag::Player))
                        .with(Collision::new())
                        .with(CheckCollision)
                        .with(Push)
                        .with(animations_container_from_file(
                            resource("animations/player.ron"),
                            spritesheet_handle.clone(),
                        ))
                        .with(Flipped::None)
                        .with(Harmable)
                        .build()
                        .id();
                }
            }

            // let player = data
            //     .world
            //     .create_entity()
            //     .with(Player::new(settings.player.clone()))
            //     .with(transform.clone())
            //     .with(sprite_render)
            //     .with(Transparent)
            //     .with(Velocity::default())
            //     // .with(MaxVelocity::from(settings.player.max_velocity)) // TODO
            //     .with(DecreaseVelocity::from(settings.player.decr_velocity))
            //     .with(size.clone())
            //     .with(ScaleOnce)
            //     .with(Gravity::from(settings.player.gravity))
            //     .with(Solid::new(SolidTag::Player))
            //     .with(Collision::new())
            //     .with(CheckCollision)
            //     .with(Push)
            //     .with(animations_container_from_file(
            //         resource("animations/player.ron"),
            //         spritesheet_handle.clone(),
            //     ))
            //     .with(Flipped::None)
            //     .with(Harmable)
            //     .build();
            // self.player_id = Some(player.id());
            self.player_id = Some(player_id);

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
                .with(animations_container_from_file(
                    resource("animations/player_attack.ron"),
                    spritesheet_handle,
                ))
                .with(Flipped::None)
                .with(Hidden)
                .build();
        }
    }

    fn build_camera(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        let settings = data.world.settings();

        let mut transform = Transform::default();
        transform.set_z(CAMERA_Z);

        let mut camera = Camera::new()
            .base_speed({ settings.camera.base_speed })
            .deadzone({ settings.camera.deadzone });

        if let Some(player_id) = self.player_id {
            camera = camera.follow(player_id);
        }

        let window_size = {
            let dim = data
                .data
                .custom
                .as_ref()
                .unwrap()
                .display_config
                .dimensions
                .unwrap();
            (dim.0 as f32, dim.1 as f32)
        };
        let size = Vector::new(
            window_size.0 * settings.camera.size_mult.0,
            window_size.1 * settings.camera.size_mult.1,
        );
        let inner_size = Vector::new(
            window_size.0 * settings.camera.inner_size_mult.0,
            window_size.1 * settings.camera.inner_size_mult.1,
        );

        let mut entity_builder = data
            .world
            .create_entity()
            .with(AmethystCamera::from(Projection::orthographic(
                0.0,    // Left
                size.0, // Right
                0.0,    // Bottom (!)
                size.1, // Top    (!)
            )))
            .with(camera.build())
            .with(transform)
            .with(Size::from(size))
            .with(InnerSize(Size::from(inner_size)))
            .with(Velocity::default())
            .with(Collision::new());

        if let Some(level_size) = self.level_size {
            // NOTE: Offset the values by half of camera's size,
            // because the `ConfineEntitiesSystem` assumes the entity's
            // anchor point is in the center. The Camera is the only
            // entity for which the anchor point is in the bottom left.
            entity_builder = entity_builder.with(Confined::new(Rect {
                top:    level_size.1 - size.1 * 0.5,
                bottom: 0.0 - size.1 * 0.5,
                left:   0.0 - size.0 * 0.5,
                right:  level_size.0 - size.0 * 0.5,
            }));
        }

        let entity = entity_builder.build();

        self.camera_id = Some(entity.id());
    }

    fn build_tiles(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
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

            let (sprite_render_opt, animation_opt) =
                if let Some(Graphic::Sprite(sprite_data)) = graphic {
                    let (sprite_render, animation_opt) = {
                        let spritesheet_handle = data
                            .world
                            .write_resource::<SpriteSheetHandles>()
                            .get_or_load(
                                &sprite_data.spritesheet_path,
                                &data.world,
                            );
                        (
                            SpriteRender {
                                sprite_sheet:  spritesheet_handle.clone(),
                                sprite_number: sprite_data.sprite_id,
                            },
                            animation_from(spritesheet_handle, &properties),
                        )
                    };
                    (Some(sprite_render), animation_opt)
                } else {
                    (None, None)
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

            if let Some(animation) = animation_opt {
                entity = entity.with(animation);
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

            // Only add the `Loadable` component if `always_loaded` is ommited or `false`.
            if let Some(false) | None = properties["always_loaded"].as_bool() {
                entity = entity.with(Loadable);
            }

            if let Some(is_solid) = properties["solid"].as_bool() {
                if is_solid {
                    entity = entity
                        .with(Solid::new(SolidTag::default()))
                        .with(Collision::new());
                }
            }

            if let Some(harmful_damage) = properties["harmful"].as_u32() {
                entity = entity
                    .with(Collision::new())
                    .with(Harmful::with_damage(harmful_damage));
            }

            entity.build();
        }
    }

    fn build_parallax(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
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
                let mut parallax_repeat_opt = None;

                let mut has_set_offset = false;

                for (key, val) in properties.entries() {
                    match key {
                        "speed_mult" => {
                            parallax = parallax.speed_mult(
                                parse_string_to_vector(val.as_str().expect(
                                    "Couldn't parse JsonValue as string \
                                     (speed_mult)",
                                )),
                            );
                        }
                        "speed_mult_x" => {
                            parallax =
                                parallax.speed_mult_x(val.as_f32().expect(
                                    "Couldn't parse JsonValue as f32 \
                                     (speed_mult_x)",
                                ));
                        }
                        "speed_mult_y" => {
                            parallax =
                                parallax.speed_mult_y(val.as_f32().expect(
                                    "Couldn't parse JsonValue as f32 \
                                     (speed_mult_y)",
                                ));
                        }
                        "offset" => {
                            has_set_offset = true;
                            parallax = parallax.offset(parse_string_to_vector(
                                val.as_str().expect(
                                    "Couldn't parse JsonValue as string \
                                     (offset)",
                                ),
                            ));
                        }
                        "offset_x" => {
                            parallax = parallax.offset_x(val.as_f32().expect(
                                "Couldn't parse JsonValue as f32 (offset_x)",
                            ));
                        }
                        "offset_y" => {
                            parallax = parallax.offset_y(val.as_f32().expect(
                                "Couldn't parse JsonValue as f32 (offset_y)",
                            ));
                        }
                        "image" => {
                            if let Some(texture_handle) = &texture_handle_opt {
                                entity = entity.with(texture_handle.clone());
                            }
                        }
                        "repeat_x" => {
                            if val.as_bool().expect(
                                "Couldn't parse JsonValue as bool (repeat_x)",
                            ) {
                                parallax_repeat_opt
                                    .get_or_insert(ParallaxRepeat::default())
                                    .repeat_x = true;
                            } else {
                                parallax_repeat_opt
                                    .get_or_insert(ParallaxRepeat::default())
                                    .repeat_x = false;
                            }
                        }
                        "repeat_y" => {
                            if val.as_bool().expect(
                                "Couldn't parse JsonValue as bool (repeat_y)",
                            ) {
                                parallax_repeat_opt
                                    .get_or_insert(ParallaxRepeat::default())
                                    .repeat_y = true;
                            } else {
                                parallax_repeat_opt
                                    .get_or_insert(ParallaxRepeat::default())
                                    .repeat_y = false;
                            }
                        }
                        _ => (),
                    }
                }

                // Set offset as parallax object position, unless 'offset' property was given
                if !has_set_offset {
                    // The offset should be the position seen in tiled;
                    // `pos` is the center of the object, but we want the top-left corner
                    let offset =
                        Vector::new(pos.0 - size.0 * 0.5, pos.1 + size.1 * 0.5);
                    parallax = parallax.offset(offset);
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

                if let Some(parallax_repeat) = parallax_repeat_opt {
                    entity = entity.with(parallax_repeat);
                }

                entity.build();
            }
        }
    }

    fn build_enemies(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
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
                flipped_opt,
            ) = enemy_components_from(&mut data.world, properties);

            let mut transform = Transform::default();
            transform.set_xyz(
                pos.0,
                pos.1,
                properties[PROPERTY_Z_KEY].as_f32().unwrap_or(ENEMY_Z),
            );

            let heart_size = Vector::new(16.0, 16.0);

            let mut entity = data
                .world
                .create_entity()
                .with(transform)
                .with(Size::from(*size))
                .with(Velocity::default())
                .with(DecreaseVelocity::from(enemy_settings.decr_velocity))
                .with(Collision::new())
                .with(CheckCollision)
                .with(Solid::new(SolidTag::Enemy))
                .with(ScaleOnce)
                .with(
                    HeartsContainer::new()
                        .health(enemy_settings.health)
                        .heart_offset(Vector::new(
                            0.0,
                            size.1 * -0.5 + heart_size.1 * -0.5,
                        ))
                        .heart_size(heart_size)
                        .build(),
                )
                .with(Enemy::new(enemy_type.clone(), enemy_settings))
                .with(sprite_render)
                .with(flipped_opt.unwrap_or(Flipped::None))
                .with(animations_container)
                .with(Transparent)
                .with(enemy_ai)
                .with(Harmable)
                .with(Loadable);

            if enemy_type != EnemyType::Flying
                && enemy_type != EnemyType::Turret
            {
                entity = entity.with(Gravity::from(settings.enemies.gravity));
            }

            entity.build();
        }
    }

    fn build_goal(&mut self, data: &mut StateData<CustomGameData<CustomData>>) {
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

    fn build_items(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
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

            let heart_size = Vector::new(16.0, 16.0);

            data.world
                .create_entity()
                .with(
                    HeartsContainer::new()
                        .health(item.cost)
                        .heart_offset(Vector::new(
                            0.0,
                            size.1 * 0.5 + heart_size.1 * 0.5,
                        ))
                        .heart_size(heart_size)
                        .build(),
                )
                .with(item)
                .with(transform)
                .with(Size::from(*size))
                .with(ScaleOnce)
                .with(Collision::new())
                .with(sprite_render)
                .with(Transparent)
                .with(Loadable)
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
