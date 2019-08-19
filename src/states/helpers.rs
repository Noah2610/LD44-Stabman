use std::collections::HashMap;

use amethyst::ui::{Anchor as AmethystAnchor, UiTransform};
use climer::Timer;

#[derive(Default)]
pub struct ToMainMenu(pub bool);

#[derive(Default)]
pub struct UpdateHealthDisplay(pub bool);

#[derive(Default)]
pub struct Timers {
    pub level:  Timer,
    pub global: Option<Timer>,
}

pub enum CampaignType {
    Normal,
    Bonus,
}

impl Default for CampaignType {
    fn default() -> Self {
        CampaignType::Normal
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub deaths: StatsDeaths,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct StatsDeaths(HashMap<String, StatsLevelDeaths>);

impl StatsDeaths {
    pub fn add_for<T>(&mut self, level: T)
    where
        T: ToString,
    {
        let level = level.to_string();
        let level_deaths = self.0.entry(level).or_insert_with(Default::default);
        level_deaths.current += 1;
        level_deaths.total += 1;
    }

    pub fn reset_current(&mut self) {
        for (_, level_deaths) in self.0.iter_mut() {
            level_deaths.current = 0;
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct StatsLevelDeaths {
    pub current: u32,
    pub total:   u32,
}

/// `UiTransform::new` wrapper
pub fn new_ui_transform<T: ToString>(
    name: T,
    anchor: AmethystAnchor,
    pos: (f32, f32, f32, f32, f32, i32),
) -> UiTransform {
    UiTransform::new(
        name.to_string(),
        anchor,
        pos.0, // x
        pos.1, // y
        pos.2, // z
        pos.3, // width
        pos.4, // height
        pos.5, // tab-order (?)
    )
}
