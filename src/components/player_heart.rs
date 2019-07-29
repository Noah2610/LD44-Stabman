use super::component_prelude::*;
use super::Heart;

pub struct PlayerHeart(pub Heart);

impl Component for PlayerHeart {
    type Storage = VecStorage<Self>;
}
