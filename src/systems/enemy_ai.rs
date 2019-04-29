use deathframe::geo::Vector;

use super::system_prelude::*;

pub struct EnemyAiSystem;

impl<'a> System<'a> for EnemyAiSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, EnemyAi>,
        ReadStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Flipped>,
    );

    fn run(
        &mut self,
        (
            time,
            players,
            enemies,
            enemy_ais,
            transforms,
            mut velocities,
            mut flippeds,
        ): Self::SystemData,
    ) {
        if let Some(player_data) =
            (&players, &transforms)
                .join()
                .find_map(|(player, transform)| {
                    if player.in_control {
                        Some(PlayerData {
                            player,
                            pos: transform.into(),
                        })
                    } else {
                        None
                    }
                })
        {
            let dt = time.delta_seconds();

            for (
                enemy,
                enemy_ai,
                enemy_transform,
                enemy_velocity,
                enemy_flipped,
            ) in (
                &enemies,
                &enemy_ais,
                &transforms,
                &mut velocities,
                &mut flippeds,
            )
                .join()
            {
                match enemy_ai {
                    EnemyAi::Tracer => run_for_tracer_ai(
                        dt,
                        &player_data,
                        enemy,
                        enemy_transform,
                        enemy_velocity,
                    ),
                }

                // Flip sprite
                if enemy_velocity.x > 0.0
                    && enemy_flipped == &mut Flipped::Horizontal
                {
                    *enemy_flipped = Flipped::None;
                } else if enemy_velocity.x < 0.0
                    && enemy_flipped == &mut Flipped::None
                {
                    *enemy_flipped = Flipped::Horizontal;
                }
            }
        }
    }
}

fn run_for_tracer_ai<'a>(
    dt: f32,
    player_data: &PlayerData,
    enemy: &Enemy,
    transform: &Transform,
    velocity: &mut Velocity,
) {
    let enemy_pos = Vector::from(transform);
    let distance_to_player = (
        enemy_pos.0 - player_data.pos.0,
        enemy_pos.1 - player_data.pos.1,
    );

    if enemy.in_trigger_distance(distance_to_player.into()) {
        velocity.x +=
            enemy.acceleration.0 * -distance_to_player.0.signum() * dt;
        if enemy.acceleration.1 != 0.0 {
            velocity.y +=
                enemy.acceleration.1 * -distance_to_player.1.signum() * dt;
        }
    }
}

struct PlayerData<'a> {
    pub player: &'a Player,
    pub pos:    Vector,
}
