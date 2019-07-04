use crate::components::prelude::*;
use deathframe::geo::Vector;

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

    pub fn push(&mut self, bullet_components: BulletComponents) {
        self.0.push(bullet_components);
    }
}
