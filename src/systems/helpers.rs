use super::system_prelude::*;

pub use direction::*;

mod direction {
    use std::convert::TryFrom;

    const ACTION_DASH_UP_LEFT: &str = "player_dash_up_left";
    const ACTION_DASH_UP_RIGHT: &str = "player_dash_up_right";
    const ACTION_DASH_DOWN_LEFT: &str = "player_dash_down_left";
    const ACTION_DASH_DOWN_RIGHT: &str = "player_dash_down_right";
    const ACTION_DASH_UP: &str = "player_dash_up";
    const ACTION_DASH_DOWN: &str = "player_dash_down";
    const ACTION_DASH_LEFT: &str = "player_dash_left";
    const ACTION_DASH_RIGHT: &str = "player_dash_right";

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

#[derive(Default, Clone)]
pub struct SidesTouching {
    pub is_touching_top:    bool,
    pub is_touching_bottom: bool,
    pub is_touching_left:   bool,
    pub is_touching_right:  bool,
}

impl<'a> SidesTouching {
    pub fn new(
        entities: &Entities<'a>,
        player_collision: &Collision,
        collisions: &ReadStorage<'a, Collision>,
        solids: &ReadStorage<Solid<SolidTag>>,
    ) -> Self {
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

    // cause im a stupid boy
    pub fn new_with_collisions_mut(
        entities: &Entities<'a>,
        player_collision: &Collision,
        collisions: &WriteStorage<'a, Collision>,
        solids: &ReadStorage<Solid<SolidTag>>,
    ) -> Self {
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
