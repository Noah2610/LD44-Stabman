use super::state_prelude::*;
use amethyst::assets::ProgressCounter;
use amethyst::ecs::{Join, ReadStorage, WriteStorage};
use amethyst::ui::{UiText, UiTransform};
use std::collections::HashMap;

const UI_RON_PATH: &str = "ui/win_game_menu.ron";

#[derive(Default)]
pub struct WinGameMenu {
    campaign:            CampaignType,
    ui_entities:         Vec<Entity>,
    ui_reader_id:        Option<ReaderId<UiEvent>>,
    ui_creator_progress: Option<ProgressCounter>,
}

impl WinGameMenu {
    pub fn new(campaign: CampaignType) -> Self {
        Self {
            campaign:            campaign,
            ui_entities:         Vec::new(),
            ui_reader_id:        None,
            ui_creator_progress: None,
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
        // Continue game - switch to Ingame again
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

    fn populate_ui_texts(
        &self,
        mut data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        let mut stats_texts = HashMap::<String, String>::new();

        let level_manager = {
            let settings = data.world.settings();
            let level_manager_settings = match self.campaign {
                CampaignType::Normal => settings.level_manager.normal,
                CampaignType::BonusA => settings.level_manager.bonus_a,
                CampaignType::BonusB => settings.level_manager.bonus_b,
            };
            LevelManager::new(&mut data, level_manager_settings, false)
        };

        // Times - current
        {
            let timers = data.world.read_resource::<Timers>();
            if let Some(global_time) = &timers.global {
                stats_texts.insert(
                    "stats_global_time_current".to_string(),
                    global_time.time_output().to_string(),
                );
            }
        }
        // Times - best
        if let Some(global_time_data) = level_manager.global_time {
            stats_texts.insert(
                "stats_global_time_best".to_string(),
                global_time_data.general.to_string(),
            );
        }
        // Stats
        {
            let stats = data.world.read_resource::<Stats>();
            let deaths = stats
                .levels
                .0
                .values()
                .fold(0, |acc, level| acc + level.deaths.current);
            stats_texts.insert("stats_deaths".to_string(), deaths.to_string());
        }

        data.world.exec(
            |(ui_transforms, mut ui_texts): (
                ReadStorage<UiTransform>,
                WriteStorage<UiText>,
            )| {
                for (ui_transform, ui_text) in
                    (&ui_transforms, &mut ui_texts).join()
                {
                    if let Some(append_text) = stats_texts.get(&ui_transform.id)
                    {
                        ui_text.text += append_text;
                    }
                }
            },
        );
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent>
    for WinGameMenu
{
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        self.ui_creator_progress = Some(self.create_ui(&mut data));
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

        // TODO: Unused
        // if let Some(progress) = self.ui_creator_progress.take() {
        //     if progress.is_complete() {
        //         self.populate_ui_texts(&mut data);
        //     } else {
        //         self.ui_creator_progress = Some(progress);
        //     }
        // }

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

impl Menu for WinGameMenu {
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
            "quit_button" => Some(Trans::Pop),
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
