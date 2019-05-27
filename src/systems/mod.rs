mod bullet;
mod debug;
mod enemy_ai;
mod goal;
mod health_display;
mod player_attack;
mod player_controls;
mod player_take_damage;

pub mod prelude {
    pub use deathframe::systems::prelude::*;

    pub use super::bullet::BulletSystem;
    pub use super::debug::DebugSystem;
    pub use super::enemy_ai::EnemyAiSystem;
    pub use super::goal::GoalSystem;
    pub use super::health_display::HealthDisplaySystem;
    pub use super::player_attack::PlayerAttackSystem;
    pub use super::player_controls::PlayerControlsSystem;
    pub use super::player_take_damage::PlayerTakeDamageSystem;
}

mod system_prelude {
    pub use deathframe::geo::Side;
    pub use deathframe::systems::system_prelude::*;

    pub use super::helpers::*;
    pub use crate::components::prelude::*;
    pub use crate::solid_tag::SolidTag;
}

mod helpers {
    use super::system_prelude::*;

    #[derive(Default, Clone)]
    pub struct SidesTouching {
        pub is_touching_top:    bool,
        pub is_touching_bottom: bool,
        pub is_touching_left:   bool,
        pub is_touching_right:  bool,
    }

    impl<'a> SidesTouching {
        pub fn new(
            entities: &Entities<'a>,
            player_collision: &Collision,
            collisions: &ReadStorage<'a, Collision>,
            solids: &ReadStorage<Solid<SolidTag>>,
        ) -> Self {
            let mut is_touching_top = false;
            let mut is_touching_bottom = false;
            let mut is_touching_left = false;
            let mut is_touching_right = false;
            if player_collision.in_collision() {
                for (other_entity, _, _) in
                    (entities, collisions, solids).join()
                {
                    if let Some(colliding_with) =
                        player_collision.collision_with(other_entity.id())
                    {
                        match colliding_with.side {
                            Side::Top => is_touching_top = true,
                            Side::Bottom => is_touching_bottom = true,
                            Side::Left => is_touching_left = true,
                            Side::Right => is_touching_right = true,
                            _ => (),
                        }
                        if is_touching_top
                            && is_touching_bottom
                            && is_touching_left
                            && is_touching_right
                        {
                            break;
                        }
                    }
                }
            }
            Self {
                is_touching_top,
                is_touching_bottom,
                is_touching_left,
                is_touching_right,
            }
        }

        // cause im a stupid boy
        pub fn new_with_collisions_mut(
            entities: &Entities<'a>,
            player_collision: &Collision,
            collisions: &WriteStorage<'a, Collision>,
            solids: &ReadStorage<Solid<SolidTag>>,
        ) -> Self {
            let mut is_touching_top = false;
            let mut is_touching_bottom = false;
            let mut is_touching_left = false;
            let mut is_touching_right = false;
            if player_collision.in_collision() {
                for (other_entity, _, _) in
                    (entities, collisions, solids).join()
                {
                    if let Some(colliding_with) =
                        player_collision.collision_with(other_entity.id())
                    {
                        match colliding_with.side {
                            Side::Top => is_touching_top = true,
                            Side::Bottom => is_touching_bottom = true,
                            Side::Left => is_touching_left = true,
                            Side::Right => is_touching_right = true,
                            _ => (),
                        }
                        if is_touching_top
                            && is_touching_bottom
                            && is_touching_left
                            && is_touching_right
                        {
                            break;
                        }
                    }
                }
            }
            Self {
                is_touching_top,
                is_touching_bottom,
                is_touching_left,
                is_touching_right,
            }
        }

        pub fn is_touching_horizontally(&self) -> bool {
            self.is_touching_left || self.is_touching_right
        }

        pub fn is_touching_vertically(&self) -> bool {
            self.is_touching_top || self.is_touching_bottom
        }
    }
}

pub use prelude::*;
