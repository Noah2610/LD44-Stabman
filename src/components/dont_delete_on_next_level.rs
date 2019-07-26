use super::component_prelude::*;

#[derive(Default)]
pub struct DontDeleteOnNextLevel;

impl Component for DontDeleteOnNextLevel {
    type Storage = NullStorage<Self>;
}
