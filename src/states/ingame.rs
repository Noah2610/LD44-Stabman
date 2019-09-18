use amethyst::audio::AudioSink;

use super::state_prelude::*;

pub struct Ingame {
    campaign:          CampaignType,
    to_main_menu:      bool,
    new_game:          bool,
    should_load_level: bool,
}

impl Ingame {
    pub fn builder() -> IngameBuilder {
        IngameBuilder::default()
    }

    fn handle_keys<'a, 'b>(
        &self,
        data: &StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input_manager = data.world.input_manager();

        if input_manager.is_down("pause") {
            let paused_state = Box::new(Paused::default());
            Some(Trans::Push(paused_state))
        } else {
            None
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Ingame {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        // Initialize the LevelManager
        let mut campaign_manager =
            data.world.write_resource::<CampaignManager>();
        campaign_manager.select_campaign(
            self.campaign,
            &mut data,
            self.new_game,
        );
        self.should_load_level = true;
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        data.world
            .write_resource::<CampaignManager>()
            .stop_level(&mut data);
    }

    fn on_pause(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        if !self.should_load_level {
            // Set _lower_ music volume
            data.world
                .write_resource::<AudioSink>()
                .set_volume(data.world.settings().music_volume_paused);
        }
    }

    fn on_resume(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        if self.should_load_level {
            // Came from LoadLevel state
            self.should_load_level = false;
        } else {
            // Set _regular_ music volume
            data.world
                .write_resource::<AudioSink>()
                .set_volume(data.world.settings().music_volume);

            // Return to main menu, if `Paused` state set the resource to do so
            self.to_main_menu = data.world.read_resource::<ToMainMenu>().0;
        }
    }

    fn handle_event(
        &mut self,
        _data: StateData<CustomGameData<CustomData>>,
        event: StateEvent,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        match &event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn update(
        &mut self,
        mut data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        // Return to main menu, if necessary
        if self.to_main_menu {
            return Trans::Pop;
        }

        if self.should_load_level {
            return Trans::Push(Box::new(LoadLevel::default()));
        }

        data.data.update(&data.world, "ingame").unwrap();

        if let Some(trans) = data
            .world
            .write_resource::<CampaignManager>()
            .update_level(&mut data)
            .unwrap()
        {
            return trans;
        }

        if let Some(trans) = self.handle_keys(&data) {
            return trans;
        }

        Trans::None
    }
}

#[derive(Default)]
pub struct IngameBuilder {
    campaign:          CampaignType,
    new_game:          bool,
    should_load_level: bool,
}

impl IngameBuilder {
    pub fn campaign(mut self, campaign: CampaignType) -> Self {
        self.campaign = campaign;
        self
    }

    pub fn new_game(mut self, new_game: bool) -> Self {
        self.new_game = new_game;
        self
    }

    pub fn build(self) -> Ingame {
        Ingame {
            campaign:          self.campaign,
            to_main_menu:      false,
            new_game:          self.new_game,
            should_load_level: false,
        }
    }
}
