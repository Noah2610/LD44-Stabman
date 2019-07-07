use crate::components::Player;

#[derive(Serialize, Deserialize)]
pub struct SavefileData {
    pub player:      Player,
    pub level_index: usize,
}
