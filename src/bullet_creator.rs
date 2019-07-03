use crate::components::prelude::*;

pub mod prelude {
    pub use super::BulletComponents;
    pub use super::BulletCreator;
}

pub struct BulletComponents {
    pub bullet:    Bullet,
    pub transform: Transform,
    pub velocity:  Velocity,
    pub size:      Size,
}

#[derive(Default)]
pub struct BulletCreator(Vec<BulletComponents>);

impl BulletCreator {
    pub fn pop(&mut self) -> Option<BulletComponents> {
        self.0.pop()
    }
}
