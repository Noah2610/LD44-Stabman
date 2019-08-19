use std::collections::HashMap;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub levels: StatsLevels,
}

impl Stats {
    pub fn level_mut<T>(&mut self, level: T) -> &mut StatsLevel
    where
        T: ToString,
    {
        let level = level.to_string();
        self.levels.0.entry(level).or_insert_with(Default::default)
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct StatsLevels(HashMap<String, StatsLevel>);

impl StatsLevels {
    pub fn reset_current_deaths(&mut self) {
        for (_, level_stats) in self.0.iter_mut() {
            level_stats.deaths.current = 0;
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct StatsLevel {
    pub deaths: StatsLevelDeaths,
    // pub kills:  StatsLevelKills,
}

impl StatsLevelDeaths {
    pub fn increase(&mut self) {
        self.current += 1;
        self.total += 1;
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct StatsLevelDeaths {
    pub current: u32,
    pub total:   u32,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct StatsKills {}
