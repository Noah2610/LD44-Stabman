use std::collections::HashMap;

use climer::Time;

use crate::components::Player;
use crate::states::helpers::Stats;

#[derive(Serialize, Deserialize)]
pub struct SavefileData {
    pub player: Option<Player>,
    pub levels: LevelsData,
    pub stats:  Option<Stats>,
}

#[derive(Serialize, Deserialize)]
pub struct LevelsData {
    pub current:     String,
    pub completed:   Vec<String>,
    pub times:       HashMap<String, TimeData>,
    pub global_time: Option<TimeData>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimeData {
    pub general: Time,
    pub first:   Time,
}
