pub mod prelude {
    pub use super::campaign_type::CampaignType;
    pub use super::level_manager::prelude::*;
    pub use super::CampaignManager;
}

pub mod campaign_type;
pub mod level_manager;

use std::collections::HashMap;

use amethyst::{StateData, StateEvent, Trans};
use deathframe::custom_game_data::CustomGameData;

use crate::resources::*;
use crate::world_helpers::*;
use crate::CustomData;
use campaign_type::CampaignType;
use level_manager::LevelManager;

#[derive(Default)]
pub struct CampaignManager {
    active_campaign: Option<CampaignType>,
    level_managers:  HashMap<CampaignType, LevelManager>,
}

impl CampaignManager {
    pub fn select_campaign(
        &mut self,
        campaign: CampaignType,
        data: &mut StateData<CustomGameData<CustomData>>,
        new_game: bool,
    ) {
        match self.level_managers.get_mut(&campaign) {
            None => {
                self.level_managers.insert(
                    campaign,
                    new_level_manager(campaign, data, new_game),
                );
            }
            Some(level_manager) => {
                if new_game {
                    *level_manager =
                        new_level_manager(campaign, data, new_game);
                } else {
                    level_manager.restart_level(data);
                }
            }
        }
        self.active_campaign = Some(campaign);
    }

    pub fn load_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) -> Result<(), String> {
        let level_manager = self.active_level_manager()?;

        // Initialize global timer
        // NOTE: This needs to happen before the level loads
        if level_manager.is_first_level() {
            let mut timers = data.world.write_resource::<Timers>();
            let timer = climer::Timer::default();
            timers.global = Some(timer);
        }

        level_manager.load_current_level(&mut data);
        // Force update `HealthDisplay`
        data.world.write_resource::<UpdateHealthDisplay>().0 = true;

        // Now start the global timer
        data.world
            .write_resource::<Timers>()
            .global
            .as_mut()
            .map(|timer| timer.start().unwrap());

        Ok(())
    }

    pub fn stop_level(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) -> Result<(), String> {
        self.active_level_manager()?.stop_level(data);
        Ok(())
    }

    pub fn update_level<'a, 'b>(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) -> Result<
        Option<Trans<CustomGameData<'a, 'b, CustomData>, StateEvent>>,
        String,
    > {
        let level_manager = self.active_level_manager()?;
        level_manager.update(data);
        Ok(if level_manager.has_won_game {
            // Switch to WinGameMenu
            Some(Trans::Switch(Box::new(crate::states::WinGameMenu::new(
                self.active_campaign()?,
            ))))
        } else {
            None
        })
    }

    fn active_level_manager(&mut self) -> Result<&mut LevelManager, String> {
        let campaign = self.active_campaign()?;
        if let Some(lm) = self.level_managers.get_mut(&campaign) {
            Ok(lm)
        } else {
            Err(String::from(
                "LevelManager doesn't exist for active campaign. This \
                 shouldn't happen :o",
            ))
        }
    }

    fn active_campaign(&self) -> Result<CampaignType, String> {
        if let Some(c) = self.active_campaign {
            Ok(c)
        } else {
            Err(String::from(
                "No active campaign. Select a campaign first from \
                 CampaignManager with `select_campaign`",
            ))
        }
    }
}

fn new_level_manager(
    campaign: CampaignType,
    data: &mut StateData<CustomGameData<CustomData>>,
    new_game: bool,
) -> LevelManager {
    let settings = data.world.settings();
    let level_manager_settings = match campaign {
        CampaignType::Normal => settings.level_manager.normal,
        CampaignType::BonusA => settings.level_manager.bonus_a,
        CampaignType::BonusB => settings.level_manager.bonus_b,
    };
    LevelManager::new(&mut data, level_manager_settings, new_game)
}
