use super::component_prelude::*;

#[derive(Default)]
pub struct Invincible;

impl Component for Invincible {
    type Storage = NullStorage<Self>;
}
