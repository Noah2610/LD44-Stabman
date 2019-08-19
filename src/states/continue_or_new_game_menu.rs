use super::state_prelude::*;

const UI_RON_PATH: &str = "ui/continue_or_new_game_menu.ron";

pub struct ContinueOrNewGameMenu {
    campaign:     CampaignType,
    ui_entities:  Vec<Entity>,
    ui_reader_id: Option<ReaderId<UiEvent>>,
}

impl ContinueOrNewGameMenu {
    pub fn new(campaign: CampaignType) -> Self {
        Self {
            campaign:     campaign,
            ui_entities:  Vec::new(),
            ui_reader_id: None,
        }
    }

    fn handle_keys<'a, 'b>(
        &self,
        data: &StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input_manager = data.world.input_manager();

        // Back to main menu - pop off
        if input_manager.is_up("decline") {
            Some(Trans::Pop)
        // Continue game
        } else if input_manager.is_up("accept") {
            Some(Trans::Switch(Box::new(
                Ingame::builder()
                    .campaign(self.campaign.clone())
                    .new_game(false)
                    .build(),
            )))
        } else {
            None
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent>
    for ContinueOrNewGameMenu
{
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.create_ui(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
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
        data.data
            .update(&data.world, "continue_or_new_game_menu")
            .unwrap();
        if let Some(trans) = self.handle_keys(&data) {
            return trans;
        }
        Trans::None
    }

    fn fixed_update(
        &mut self,
        mut data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        if let Some(trans) = self.update_ui_events(&mut data) {
            return trans;
        }
        Trans::None
    }
}

impl Menu for ContinueOrNewGameMenu {
    fn event_triggered<'a, 'b>(
        &mut self,
        _data: &mut StateData<CustomGameData<CustomData>>,
        event_name: String,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        match event_name.as_ref() {
            "continue_button" => Some(Trans::Switch(Box::new(
                Ingame::builder()
                    .campaign(self.campaign.clone())
                    .new_game(false)
                    .build(),
            ))),
            "new_game_button" => Some(Trans::Switch(Box::new(
                Ingame::builder()
                    .campaign(self.campaign.clone())
                    .new_game(true)
                    .build(),
            ))),
            _ => None,
        }
    }

    fn ui_ron_path(&self) -> &str {
        UI_RON_PATH
    }

    fn ui_entities(&self) -> &Vec<Entity> {
        &self.ui_entities
    }

    fn ui_entities_mut(&mut self) -> &mut Vec<Entity> {
        &mut self.ui_entities
    }

    fn ui_reader_id(&self) -> &Option<ReaderId<UiEvent>> {
        &self.ui_reader_id
    }

    fn ui_reader_id_mut(&mut self) -> &mut Option<ReaderId<UiEvent>> {
        &mut self.ui_reader_id
    }
}
