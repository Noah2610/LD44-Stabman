use amethyst::ecs::world::Index;

use super::component_prelude::*;

pub mod prelude {
    pub use super::HeartsContainer;
    pub use super::HeartsContainerBuilder;
}

mod defaults {
    pub const HEART_SIZE: (f32, f32) = (16.0, 16.0);
    pub const HEART_PADDING: (f32, f32) = (4.0, 4.0);
}

pub struct HeartsContainer {
    pub hp:            u32,
    pub heart_ids:     Vec<Index>,
    pub heart_size:    Vector,
    pub heart_padding: Vector,
    pub heart_offset:  Vector,
}

impl HeartsContainer {
    pub fn new() -> HeartsContainerBuilder {
        HeartsContainerBuilder::default()
    }
}

#[derive(Default)]
pub struct HeartsContainerBuilder {
    hp:            Option<u32>,
    heart_size:    Option<Vector>,
    heart_padding: Option<Vector>,
    heart_offset:  Option<Vector>,
}

impl HeartsContainerBuilder {
    pub fn hp(mut self, hp: u32) -> Self {
        self.hp = Some(hp);
        self
    }

    pub fn heart_size(mut self, heart_size: Vector) -> Self {
        self.heart_size = Some(heart_size);
        self
    }

    pub fn heart_padding(mut self, padding: Vector) -> Self {
        self.heart_padding = Some(padding);
        self
    }

    pub fn heart_offset(mut self, offset: Vector) -> Self {
        self.heart_offset = Some(offset);
        self
    }

    pub fn build(self) -> HeartsContainer {
        HeartsContainer {
            hp:            self.hp.expect("HeartsContainer needs hp u32"),
            heart_ids:     Vec::new(),
            heart_size:    self
                .heart_size
                .unwrap_or(defaults::HEART_SIZE.into()),
            heart_padding: self
                .heart_padding
                .unwrap_or(defaults::HEART_PADDING.into()),
            heart_offset:  self.heart_offset.unwrap_or(Vector::new(0.0, 0.0)),
        }
    }
}

impl Component for HeartsContainer {
    type Storage = VecStorage<Self>;
}
