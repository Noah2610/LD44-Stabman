use super::component_prelude::*;

#[derive(Default)]
pub struct NoAttack;

impl Component for NoAttack {
    type Storage = NullStorage<Self>;
}
