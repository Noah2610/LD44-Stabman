use super::component_prelude::*;

#[derive(Default)]
pub struct Goal {
    pub next_level: bool,
}

impl Component for Goal {
    type Storage = HashMapStorage<Self>;
}
