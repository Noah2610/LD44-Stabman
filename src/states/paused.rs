use super::state_prelude::*;

#[derive(Default)]
pub struct Paused {}

impl Paused {
    fn handle_keys<'a, 'b>(
        &self,
        data: &StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input_manager = data.world.input_manager();

        if input_manager.is_up("quit") {
            Some(Trans::Quit)
        } else if input_manager.is_down("pause") {
            Some(Trans::Pop)
        } else {
            None
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Paused {
    fn on_start(&mut self, data: StateData<CustomGameData<CustomData>>) {

    }

    fn handle_event(
        &mut self,
        _data: StateData<CustomGameData<CustomData>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(&data.world, "paused").unwrap();

        if let Some(trans) = self.handle_keys(&data) {
            return trans;
        }

        Trans::None
    }
}
