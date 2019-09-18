use super::state_prelude::*;

#[derive(Default)]
pub struct LoadLevel {
    finished_loading: bool,
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent>
    for LoadLevel
{
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        println!("LOADING LEVEL");
        data.world
            .write_resource::<CampaignManager>()
            .load_level(&mut data)
            .expect(
                "Couldn't load level with CampaignManager. Was a campaign \
                 selected already?",
            );
        self.finished_loading = true;
    }

    fn on_stop(&mut self, data: StateData<CustomGameData<CustomData>>) {
        println!("FINISHED LOADING LEVEL");
    }

    fn update(
        &mut self,
        data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(&data.world, "load_level").unwrap();

        if self.finished_loading {
            return Trans::Pop;
        }

        Trans::None
    }
}
