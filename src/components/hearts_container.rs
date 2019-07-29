use amethyst::ecs::world::Index;

use super::component_prelude::*;

pub struct HeartsContainer {
    pub hp:        u32,
    pub heart_ids: Vec<Index>,
}

impl HeartsContainer {
    pub fn new(hp: u32) -> Self {
        Self {
            hp,
            heart_ids: Vec::new(),
        }
    }
}

impl Component for HeartsContainer {
    type Storage = VecStorage<Self>;
}
