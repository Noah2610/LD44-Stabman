use crate::components::Player;

#[derive(Serialize, Deserialize)]
pub struct SavefileData {
    pub player: Player,
    pub levels: LevelsData,
}

#[derive(Serialize, Deserialize)]
pub struct LevelsData {
    pub current:   String,
    pub completed: Vec<String>,
}
