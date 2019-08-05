use super::component_prelude::*;

#[derive(Default)]
pub struct Noclip;

impl Component for Noclip {
    type Storage = NullStorage<Self>;
}
