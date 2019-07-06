use super::component_prelude::*;

pub struct Harmful {
    pub damage: u32,
}

impl Harmful {
    pub fn with_damage(damage: u32) -> Self {
        Self { damage }
    }
}

impl Component for Harmful {
    type Storage = VecStorage<Self>;
}
