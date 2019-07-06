use super::component_prelude::*;

#[derive(Default)]
pub struct Harmable;

impl Component for Harmable {
    type Storage = NullStorage<Self>;
}
