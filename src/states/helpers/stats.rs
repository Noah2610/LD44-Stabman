use std::collections::HashMap;

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
