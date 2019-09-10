use super::state_prelude::*;

const UI_RON_PATH: &str = "ui/bonus_select_menu.ron";

#[derive(Default)]
pub struct BonusSelectMenu {
    ui_entities:  Vec<Entity>,
    ui_reader_id: Option<ReaderId<UiEvent>>,
}

impl BonusSelectMenu {
    fn handle_keys<'a, 'b>(
        &self,
        data: &StateData<CustomGameData<CustomData>>,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let settings = data.world.settings();
        let input_manager = data.world.input_manager();

        // Back to main menu
        if input_manager.is_up("decline") {
            Some(Trans::Pop)
        // Start bonus_a
        } else if input_manager.is_up("accept") {
            Some(self.trans_for_campaign(CampaignType::BonusA, &settings))
        } else {
            None
        }
    }

    /// Returns the `Trans::Push` with the appropriate state, given the campaign.
    /// Checks if the savefile for that campaign exists;
    /// if it does exist, then push the `ContinueOrNewGameMenu` state,
    /// if it does _not_ exist, then push the `Ingame` state.
    fn trans_for_campaign<'a, 'b>(
        &self,
        campaign: CampaignType,
        settings: &Settings,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        // Only start ContinueOrNewGameMenu if a savefile already exists,
        // otherwise start the game directly.
        if savefile_exists_for(&campaign, settings) {
            Trans::Switch(Box::new(ContinueOrNewGameMenu::new(campaign)))
        } else {
            Trans::Switch(Box::new(
                Ingame::builder().campaign(campaign).new_game(false).build(),
            ))
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent>
    for BonusSelectMenu
{
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.create_ui(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.delete_ui(&mut data);
    }

    fn on_resume(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.create_ui(&mut data);
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
        data.data.update(&data.world, "bonus_select_menu").unwrap();
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

impl Menu for BonusSelectMenu {
    fn event_triggered<'a, 'b>(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
        event_name: String,
    ) -> Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>> {
        let settings = data.world.settings();
        let mut start_with_campaign = None;

        match event_name.as_ref() {
            "start_button_bonus_a" => {
                start_with_campaign = Some(CampaignType::BonusA);
            }
            "start_button_bonus_b" => {
                start_with_campaign = Some(CampaignType::BonusB);
            }
            "back_button" => return Some(Trans::Pop),
            _ => (),
        };

        if let Some(campaign) = start_with_campaign {
            Some(self.trans_for_campaign(campaign, &settings))
        } else {
            None
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

fn savefile_exists_for(campaign: &CampaignType, settings: &Settings) -> bool {
    use amethyst::utils::application_root_dir;
    use std::path::Path;

    let savefile_relative = match campaign {
        CampaignType::Normal => &settings.level_manager.normal.savefile_path,
        CampaignType::BonusA => &settings.level_manager.bonus_a.savefile_path,
        CampaignType::BonusB => &settings.level_manager.bonus_b.savefile_path,
    };
    let savefile_path =
        format!("{}/{}", application_root_dir(), savefile_relative);
    Path::new(&savefile_path).exists()
}
