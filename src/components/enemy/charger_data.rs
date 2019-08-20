use super::Facing;
use deathframe::geo::{Side, Vector};

#[derive(Clone, PartialEq, Default)]
pub struct EnemyAiChargerData {
    pub is_moving:                        bool,
    pub velocity:                         Vector,
    pub stop_moving_when_colliding_sides: Option<Vec<Side>>,
}
