use super::component_prelude::*;

#[derive(Default)]
pub struct Loader {
    pub distance: Option<Vector>,
}

impl Component for Loader {
    type Storage = VecStorage<Self>;
}
