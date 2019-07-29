use amethyst::ecs::world::Index;

use super::component_prelude::*;

const HEART_SIZE: (f32, f32) = (16.0, 16.0);
const HEART_PADDING: (f32, f32) = (16.0, 16.0);

pub struct HeartsContainer {
    pub hp:            u32,
    pub heart_ids:     Vec<Index>,
    pub heart_size:    Vector,
    pub heart_padding: Vector,
}

impl HeartsContainer {
    pub fn new(hp: u32) -> Self {
        Self {
            hp,
            heart_ids: Vec::new(),
            heart_size: HEART_SIZE.into(), // TODO
            heart_padding: HEART_PADDING.into(), // TODO
        }
    }
}

impl Component for HeartsContainer {
    type Storage = VecStorage<Self>;
}
