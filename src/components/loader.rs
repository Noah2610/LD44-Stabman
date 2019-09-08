use super::component_prelude::*;

#[derive(Default)]
pub struct Loader {
    pub distance: Option<Vector>,
    pub padding:  Option<Vector>, // Extra padding beyond the entitiy's size or normal loading distance.
}

impl Component for Loader {
    type Storage = VecStorage<Self>;
}
