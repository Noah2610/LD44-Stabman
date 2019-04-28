use super::component_prelude::*;

/// Separate entity for the player's attack
#[derive(Default)]
pub struct PlayerAttack {
    pub active: bool,
}

impl Component for PlayerAttack {
    type Storage = HashMapStorage<Self>;
}
