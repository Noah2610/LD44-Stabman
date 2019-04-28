use super::component_prelude::*;

pub struct Heart {
    pub index: u32,
}

impl Heart {
    pub fn new(index: u32) -> Self {
        Self { index }
    }
}

impl Component for Heart {
    type Storage = VecStorage<Self>;
}
