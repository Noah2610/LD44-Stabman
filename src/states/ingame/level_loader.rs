use std::fs::File;
use std::io::prelude::*;

use amethyst::ecs::world::Index;
use deathframe::geo::{Anchor, Vector};
use json::JsonValue;

use super::super::state_prelude::*;
use crate::components::prelude::*;
use crate::settings::SettingsLevelManager;

const PROPERTY_Z_KEY: &str = "z";
const PLAYER_Z: f32 = 0.5;
const CAMERA_Z: f32 = 10.0;
const TILE_Z: f32 = 0.0;
const PARALLAX_Z: f32 = -1.0;
const SPRITESHEET_DIR: &str = "spritesheets";
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
    camera_id:     Option<Index>,
    player_id:     Option<Index>,
    player_data:   Option<EntityData>,
    tiles_data:    Vec<EntityData>,
    parallax_data: Vec<EntityData>,
}

impl LevelLoader {
    pub fn new(settings: SettingsLevelManager) -> Self {
        Self {
            settings:      settings,
            camera_id:     None,
            player_id:     None,
            player_data:   None,
            tiles_data:    Vec::new(),
            parallax_data: Vec::new(),
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

        self.load_objects(&json["objects"]);
        self.load_tiles(&json["tiles"]);
    }

    /// Builds the loaded data using the given `StateData`.
    pub fn build<T>(&mut self, data: &mut StateData<CustomGameData<T>>) {
        self.build_player(data);
        self.build_camera(data);
        self.build_tiles(data);
        self.build_parallax(data);
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
                match obj_type {
                    "Player" => {
                        self.player_data = Some(EntityData {
                            pos:        (x, y).into(),
                            size:       (w, h).into(),
                            properties: properties.clone(),
                            graphic:    None,
                        })
                    }
                    "Parallax" => self.parallax_data.push(EntityData {
                        pos:        (x, y).into(),
                        size:       (w, h).into(),
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
                let spritesheet_path = resource(format!(
                    "{}/{}.png",
                    SPRITESHEET_DIR, tileset_name
                ));

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
                "{}/{}",
                SPRITESHEET_DIR, PLAYER_SPRITESHEET_FILENAME
            ));
            let (spritesheet_handle, sprite_render) = {
                let spritesheet_handle = data
                    .world
                    .write_resource::<SpriteSheetHandles>()
                    .get_or_load(spritesheet_path, &data.world);
                (spritesheet_handle.clone(), SpriteRender {
                    sprite_sheet:  spritesheet_handle,
                    sprite_number: 0,
                })
            };

            let player = data
                .world
                .create_entity()
                .with(Player::new(settings.player.clone()))
                .with(transform)
                .with(sprite_render)
                .with(Transparent)
                .with(Velocity::default())
                .with(MaxVelocity::from(settings.player.max_velocity))
                .with(DecreaseVelocity::from(settings.player.decr_velocity))
                .with(size)
                .with(ScaleOnce)
                .with(Gravity::from(settings.player.gravity))
                .with(Solid)
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
                                .default_delay_ms(500)
                                .sprite_ids(vec![8])
                                .build(),
                        )
                        .insert(
                            "attack",
                            Animation::new()
                                .default_sprite_sheet_handle(spritesheet_handle)
                                .default_delay_ms(500)
                                .sprite_ids(vec![9, 11])
                                .build(),
                        )
                        .current("idle")
                        .build(),
                )
                .with(Flipped::None)
                .build();
            self.player_id = Some(player.id());
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

        let entity = data
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
            .with(Collision::new())
            .build();

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
                    entity = entity.with(Solid).with(Collision::new());
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

                for (key, val) in properties.entries() {
                    match (key, &texture_handle_opt) {
                        ("speed_mult", _) => {
                            parallax = parallax.speed_mult(
                                parse_string_to_vector(val.as_str().expect(
                                    "Couldn't parse JsonValue as string",
                                )),
                            );
                        }
                        ("offset", _) => {
                            parallax = parallax.offset(parse_string_to_vector(
                                val.as_str().expect(
                                    "Couldn't parse JsonValue as string",
                                ),
                            ))
                        }
                        ("image", Some(texture_handle)) => {
                            entity = entity.with(texture_handle.clone())
                        }
                        _ => (),
                    }
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
