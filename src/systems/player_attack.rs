use deathframe::geo::Vector;

use super::system_prelude::*;

pub struct PlayerAttackSystem;

impl<'a> System<'a> for PlayerAttackSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Collision>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, PlayerAttack>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, AnimationsContainer>,
        WriteStorage<'a, Flipped>,
        WriteStorage<'a, Hidden>,
        WriteStorage<'a, Enemy>,
    );

    fn run(
        &mut self,
        (
            entities,
            sizes,
            collisions,
            mut players,
            mut player_attacks,
            mut transforms,
            mut velocities,
            mut animations_containers,
            mut flippeds,
            mut hiddens,
            mut enemies,
        ): Self::SystemData,
    ) {
        // Get some player data
        let player_data_opt: Option<(_, _, Vector, _)> =
            (&mut players, &transforms, &sizes, &flippeds)
                .join()
                .find_map(|(player, transform, size, flipped)| {
                    Some((
                        player.is_attacking,
                        Vector::from(transform),
                        size.into(),
                        flipped.clone(),
                    ))
                });

        // Deactivate PlayerAttack if neccessary
        for (player, animations_container) in
            (&mut players, &animations_containers).join()
        {
            if animations_container.play_once.is_none() {
                player.is_attacking = false;
            }
        }

        // PlayerAttack logic
        if let Some((is_attacking, player_pos, player_size, player_flipped)) =
            player_data_opt
        {
            let mut attack_id_opt = None;

            for (
                attack_entity,
                attack,
                attack_transform,
                attack_animations_container,
                attack_flipped,
            ) in (
                &entities,
                &mut player_attacks,
                &mut transforms,
                &mut animations_containers,
                &mut flippeds,
            )
                .join()
            {
                attack_id_opt = Some(attack_entity.id());

                if is_attacking {
                    // Move PlayerAttack's transform
                    let mut pos = player_pos.clone();
                    match player_flipped {
                        Flipped::None => {
                            pos.0 += player_size.0;
                        }
                        Flipped::Horizontal => {
                            pos.0 -= player_size.0;
                        }
                        _ => (),
                    }
                    if *attack_flipped != player_flipped {
                        *attack_flipped = player_flipped.clone();
                    }
                    attack_transform.set_x(pos.0);
                    attack_transform.set_y(pos.1);
                    attack.active = true;
                    attack_animations_container.play("attack_default");
                    hiddens.remove(attack_entity);
                } else {
                    // Hacky: move PlayerAttack way off screen, so collision data is unset
                    attack_transform.set_x(-1000.0);
                    attack_transform.set_y(-1000.0);
                    attack.active = false;
                    attack_animations_container.play_once = None;
                    hiddens.insert(attack_entity, Hidden).unwrap();
                }
            }

            // Actual attacking logic
            if let Some(attack_id) = attack_id_opt {
                // Enemies to remove, below
                let mut enemies_to_delete = Vec::new();

                for player in (&mut players).join() {
                    for (attack, attack_collision, player_flipped) in
                        (&player_attacks, &collisions, &flippeds).join()
                    {
                        for (
                            enemy_entity,
                            enemy,
                            enemy_velocity,
                            enemy_animations_container,
                        ) in (
                            &entities,
                            &mut enemies,
                            &mut velocities,
                            &mut animations_containers,
                        )
                            .join()
                        {
                            let enemy_id = enemy_entity.id();
                            // Attack enemy
                            if attack.active {
                                if let Some(collision::Data {
                                    side: Side::Inner,
                                    state: collision::State::Enter,
                                    ..
                                }) =
                                    attack_collision.collision_with(enemy_id)
                                {
                                    player.deal_damage_to(enemy);
                                    // Knockback
                                    if player.items_data.has_knockback {
                                        enemy_velocity.x =
                                            player.items_data.knockback.0
                                                * match player_flipped {
                                                    Flipped::None => 1.0,
                                                    Flipped::Horizontal => -1.0,
                                                    _ => 1.0,
                                                };
                                        enemy_velocity.y =
                                            player.items_data.knockback.1;
                                    }
                                    if enemy.is_dead() {
                                        player.gain_reward(enemy.reward);
                                        enemies_to_delete.push(enemy_entity);
                                    }
                                }
                            }
                        }
                    }
                }

                // Remove killed enemies
                for enemy in enemies_to_delete {
                    entities.delete(enemy).unwrap();
                }
            }
        }
    }
}
