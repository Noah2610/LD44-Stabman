use deathframe::geo::Vector;

use super::system_prelude::*;

pub struct PlayerAttackSystem;

impl<'a> System<'a> for PlayerAttackSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, PlayerAttack>,
        WriteStorage<'a, Transform>,
        WriteStorage<'a, AnimationsContainer>,
        WriteStorage<'a, Flipped>,
        WriteStorage<'a, Hidden>,
    );

    fn run(
        &mut self,
        (
            entities,
            sizes,
            mut players,
            mut player_attacks,
            mut transforms,
            mut animations_containers,
            mut flippeds,
            mut hiddens,
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

                // Play animation, set active state, insert/remove Hidden component,
                // and actual attack/hit logic
                if is_attacking {
                    attack.active = true;
                    attack_animations_container.play("attack_default");
                    hiddens.remove(attack_entity);

                // TODO: Attacking logic
                } else {
                    attack.active = false;
                    attack_animations_container.play_once = None;
                    hiddens.insert(attack_entity, Hidden).unwrap();
                }
            }
        }
    }
}
