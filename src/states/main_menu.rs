use super::state_prelude::*;

const UI_RON_PATH: &str = "ui/main_menu.ron";

#[derive(Default)]
pub struct MainMenu {
    ui_entities:  Vec<Entity>,
    ui_reader_id: Option<ReaderId<UiEvent>>,
}

impl MainMenu {
    fn handle_keys<'a, 'b>(
        &self,
        data: &StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let input_manager = data.world.input_manager();

        // Quit game
        if input_manager.is_up("decline") {
            Some(Trans::Quit)
        // Start game
        } else if input_manager.is_up("accept") {
            Some(Trans::Push(Box::new(ContinueOrNewGameMenu::new(
                CampaignType::default(),
            ))))
        } else {
            None
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent>
    for MainMenu
{
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.create_ui(&mut data);
        // Always set `ToMainMenu` resource to `false`
        data.world.write_resource::<ToMainMenu>().0 = false;
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
    }

    fn on_resume(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.create_ui(&mut data);
        // Always set `ToMainMenu` resource to `false`
        data.world.write_resource::<ToMainMenu>().0 = false;
    }

    fn on_pause(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
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
        data.data.update(&data.world, "main_menu").unwrap();
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

impl Menu for MainMenu {
    fn event_triggered<'a, 'b>(
        &mut self,
        _data: &mut StateData<CustomGameData<CustomData>>,
        event_name: String,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        match event_name.as_ref() {
            "start_button_normal" => Some(Trans::Push(Box::new(
                ContinueOrNewGameMenu::new(CampaignType::Normal),
            ))),
            "start_button_bonus" => Some(Trans::Push(Box::new(
                ContinueOrNewGameMenu::new(CampaignType::Bonus),
            ))),
            "quit_button" => Some(Trans::Quit),
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
