pub mod prelude {
    pub use super::ItemsData;
    pub use super::ItemsDataBulletShoot;
    pub use super::ItemsDataDash;
    pub use super::ItemsDataExtraJump;
    pub use super::ItemsDataKnockback;
    pub use super::ItemsDataWallJump;
}

use std::time::Duration;

use deathframe::geo::Vector;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemsData {
    pub extra_jump:   ItemsDataExtraJump,
    pub wall_jump:    ItemsDataWallJump,
    pub knockback:    ItemsDataKnockback,
    pub bullet_shoot: ItemsDataBulletShoot,
    pub dash:         ItemsDataDash,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemsDataExtraJump {
    pub extra_jumps:      u32,
    pub used_extra_jumps: u32,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemsDataWallJump {
    pub can_wall_jump: bool,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemsDataKnockback {
    pub knockback:     Vector,
    pub has_knockback: bool,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemsDataBulletShoot {
    pub can_shoot:       bool,
    pub bullet_damage:   u32,
    pub bullet_velocity: Vector,
    pub bullet_size:     Vector,
    pub bullet_lifetime: Duration,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemsDataDash {
    pub dashes:              u32,
    pub used_dashes:         u32,
    pub dash_duration_ms:    u64,
    pub dash_velocity:       Vector,
    pub dash_input_delay_ms: u64,
}
