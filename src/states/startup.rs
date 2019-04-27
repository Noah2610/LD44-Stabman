use super::state_prelude::*;

pub struct Startup {}

impl Startup {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b>, StateEvent> for Startup {
    fn update(
        &mut self,
        data: StateData<CustomGameData>,
    ) -> Trans<CustomGameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world, "startup").unwrap();

        Trans::None
    }
}
