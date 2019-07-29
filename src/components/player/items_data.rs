pub mod prelude {
    pub use super::ItemsData;
    pub use super::ItemsDataBulletDeflect;
    pub use super::ItemsDataBulletShoot;
    pub use super::ItemsDataDash;
    pub use super::ItemsDataExtraJump;
    pub use super::ItemsDataKnockback;
    pub use super::ItemsDataThrust;
    pub use super::ItemsDataWallJump;
}

use std::time::Duration;

use deathframe::geo::Vector;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemsData {
    pub extra_jump:     ItemsDataExtraJump,
    pub wall_jump:      ItemsDataWallJump,
    pub knockback:      ItemsDataKnockback,
    pub bullet_shoot:   ItemsDataBulletShoot,
    pub dash:           ItemsDataDash,
    pub bullet_deflect: ItemsDataBulletDeflect,
    pub thrust:         ItemsDataThrust,
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
    pub velocity:      Vector,
    pub has_knockback: bool,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemsDataBulletShoot {
    pub can_shoot: bool,
    pub damage:    u32,
    pub velocity:  Vector,
    pub size:      Vector,
    pub lifetime:  Duration,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemsDataDash {
    pub dashes:         u32,
    pub used_dashes:    u32,
    pub duration_ms:    u64,
    pub velocity:       Vector,
    pub input_delay_ms: u64,
    pub double_tap:     bool,
    pub is_dashing:     bool,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemsDataBulletDeflect {
    pub can_deflect:   bool,
    pub damage:        u32,
    pub velocity_mult: Vector,
    pub lifetime:      Duration,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ItemsDataThrust {
    pub can_thrust: bool,
    pub strength:   Vector,
}
