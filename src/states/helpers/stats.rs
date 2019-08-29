use std::collections::HashMap;

use crate::components::EnemyType;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub levels: StatsLevels,
    pub wins:   u32,
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
pub struct StatsLevels(pub HashMap<String, StatsLevel>);

impl StatsLevels {
    pub fn reset_current_stats(&mut self) {
        for (_, level_stats) in self.0.iter_mut() {
            level_stats.deaths.reset_current();
            level_stats.kills.reset_current();
            level_stats.items_bought.reset_current();
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct StatsLevel {
    pub deaths:       StatsLevelDeaths,
    pub kills:        StatsLevelKills,
    pub items_bought: StatsLevelItemsBought,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct StatsLevelDeaths {
    pub current: u32,
    pub total:   u32,
}

impl StatsLevelDeaths {
    pub fn increase(&mut self) {
        self.current += 1;
        self.total += 1;
    }

    pub fn reset_current(&mut self) {
        self.current = 0;
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct StatsLevelKills {
    pub current: StatsLevelKillsEnemies,
    pub total:   StatsLevelKillsEnemies,
}

impl StatsLevelKills {
    pub fn increase_for(&mut self, enemy: &EnemyType) {
        match enemy {
            EnemyType::Normal => {
                self.current.normal += 1;
                self.total.normal += 1;
            }
            EnemyType::Charger => {
                self.current.charger += 1;
                self.total.charger += 1;
            }
            EnemyType::Flying => {
                self.current.flying += 1;
                self.total.flying += 1;
            }
            EnemyType::Reaper => {
                self.current.reaper += 1;
                self.total.reaper += 1;
            }
            EnemyType::Turret => {
                self.current.turret += 1;
                self.total.turret += 1;
            }
        }
    }

    pub fn reset_current(&mut self) {
        self.current = StatsLevelKillsEnemies::default();
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct StatsLevelKillsEnemies {
    pub normal:  u32,
    pub charger: u32,
    pub flying:  u32,
    pub reaper:  u32,
    pub turret:  u32,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct StatsLevelItemsBought {
    pub current: u32,
    pub total:   u32,
}

impl StatsLevelItemsBought {
    pub fn increase(&mut self) {
        self.current += 1;
        self.total += 1;
    }

    pub fn reset_current(&mut self) {
        self.current = 0;
    }
}
