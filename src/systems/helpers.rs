use std::ops::Deref;

use amethyst::ecs::storage::{MaskedStorage, Storage};

use super::system_prelude::*;

pub use direction::*;

mod direction {
    use std::convert::TryFrom;

    pub const ACTION_DASH_TRIGGER: &str = "player_dash_trigger";
    pub const ACTION_DASH_UP_LEFT: &str = "player_dash_up_left";
    pub const ACTION_DASH_UP_RIGHT: &str = "player_dash_up_right";
    pub const ACTION_DASH_DOWN_LEFT: &str = "player_dash_down_left";
    pub const ACTION_DASH_DOWN_RIGHT: &str = "player_dash_down_right";
    pub const ACTION_DASH_UP: &str = "player_dash_up";
    pub const ACTION_DASH_DOWN: &str = "player_dash_down";
    pub const ACTION_DASH_LEFT: &str = "player_dash_left";
    pub const ACTION_DASH_RIGHT: &str = "player_dash_right";

    #[derive(Clone, Copy, PartialEq, Debug)]
    pub enum Direction {
        UpLeft,
        UpRight,
        DownLeft,
        DownRight,
        Up,
        Down,
        Left,
        Right,
    }

    impl Direction {
        pub fn action(&self) -> &'static str {
            match self {
                Direction::UpLeft => ACTION_DASH_UP_LEFT,
                Direction::UpRight => ACTION_DASH_UP_RIGHT,
                Direction::DownLeft => ACTION_DASH_DOWN_LEFT,
                Direction::DownRight => ACTION_DASH_DOWN_RIGHT,
                Direction::Up => ACTION_DASH_UP,
                Direction::Down => ACTION_DASH_DOWN,
                Direction::Left => ACTION_DASH_LEFT,
                Direction::Right => ACTION_DASH_RIGHT,
            }
        }

        pub fn iter() -> std::vec::IntoIter<Self> {
            vec![
                Direction::UpLeft,
                Direction::UpRight,
                Direction::DownLeft,
                Direction::DownRight,
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ]
            .into_iter()
        }
    }

    impl TryFrom<&str> for Direction {
        type Error = String;

        fn try_from(string: &str) -> Result<Self, Self::Error> {
            match string {
                ACTION_DASH_UP_LEFT => Ok(Direction::UpLeft),
                ACTION_DASH_UP_RIGHT => Ok(Direction::UpRight),
                ACTION_DASH_DOWN_LEFT => Ok(Direction::DownLeft),
                ACTION_DASH_DOWN_RIGHT => Ok(Direction::DownRight),
                ACTION_DASH_UP => Ok(Direction::Up),
                ACTION_DASH_DOWN => Ok(Direction::Down),
                ACTION_DASH_LEFT => Ok(Direction::Left),
                ACTION_DASH_RIGHT => Ok(Direction::Right),
                _ => Err(format!(
                    "Given string is not a valid identifier for Direction: {}",
                    string
                )),
            }
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct SidesTouching {
    pub is_touching_top:    bool,
    pub is_touching_bottom: bool,
    pub is_touching_left:   bool,
    pub is_touching_right:  bool,
}

impl SidesTouching {
    pub fn new<D>(
        entities: &Entities,
        player_collision: &Collision,
        collisions: &Storage<Collision, D>,
        solids: &ReadStorage<Solid<SolidTag>>,
    ) -> Self
    where
        D: Deref<Target = MaskedStorage<Collision>>,
    {
        let mut is_touching_top = false;
        let mut is_touching_bottom = false;
        let mut is_touching_left = false;
        let mut is_touching_right = false;
        if player_collision.in_collision() {
            for (other_entity, _, _) in (entities, collisions, solids).join() {
                if let Some(colliding_with) =
                    player_collision.collision_with(other_entity.id())
                {
                    match colliding_with.side {
                        Side::Top => is_touching_top = true,
                        Side::Bottom => is_touching_bottom = true,
                        Side::Left => is_touching_left = true,
                        Side::Right => is_touching_right = true,
                        _ => (),
                    }
                    if is_touching_top
                        && is_touching_bottom
                        && is_touching_left
                        && is_touching_right
                    {
                        break;
                    }
                }
            }
        }
        Self {
            is_touching_top,
            is_touching_bottom,
            is_touching_left,
            is_touching_right,
        }
    }

    pub fn is_touching_horizontally(&self) -> bool {
        self.is_touching_left || self.is_touching_right
    }

    pub fn is_touching_vertically(&self) -> bool {
        self.is_touching_top || self.is_touching_bottom
    }
}

pub enum AnyHeart<'a, 'b> {
    Normal(&'b mut WriteStorage<'a, Heart>),
    Player(&'b mut WriteStorage<'a, PlayerHeart>),
}

pub fn create_heart<'a, 'b>(
    entities: &Entities<'a>,
    sprite_sheet_handles: &SpriteSheetHandles,
    transforms: &mut WriteStorage<'a, Transform>,
    sizes: &mut WriteStorage<'a, Size>,
    scale_onces: &mut WriteStorage<'a, ScaleOnce>,
    any_hearts: AnyHeart<'a, 'b>,
    sprite_renders: &mut WriteStorage<'a, SpriteRender>,
    transparents: &mut WriteStorage<'a, Transparent>,
    dont_deletes_opt: Option<&mut WriteStorage<'a, DontDeleteOnNextLevel>>,
    i: u32,
    (x, y, z): (f32, f32, f32),
    size: Vector,
    sprite_id: usize,
) -> Entity {
    let entity = entities.create();

    let mut transform = Transform::default();
    transform.set_xyz(x, y, z);

    transforms.insert(entity, transform).unwrap();
    sizes.insert(entity, Size::from(size)).unwrap();
    scale_onces.insert(entity, ScaleOnce).unwrap();
    match any_hearts {
        AnyHeart::Normal(hearts) => {
            hearts.insert(entity, Heart::new(i)).unwrap();
        }
        AnyHeart::Player(hearts) => {
            hearts.insert(entity, PlayerHeart(Heart::new(i))).unwrap();
        }
    }
    sprite_renders
        .insert(entity, SpriteRender {
            sprite_sheet:  sprite_sheet_handles
                .get("player_hearts")
                .expect("Spritesheet 'player_hearts' does not exist"),
            sprite_number: sprite_id,
        })
        .unwrap();
    transparents.insert(entity, Transparent).unwrap();
    if let Some(dont_deletes) = dont_deletes_opt {
        dont_deletes
            .insert(entity, DontDeleteOnNextLevel::default())
            .unwrap();
    }

    entity
}
