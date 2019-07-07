use super::system_prelude::*;
use crate::settings::Settings;
use deathframe::geo::Vector;

pub struct HarmfulSystem;

impl<'a> System<'a> for HarmfulSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Harmful>,
        ReadStorage<'a, Harmable>,
        ReadStorage<'a, Collision>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Enemy>,
        WriteStorage<'a, Velocity>,
    );

    fn run(
        &mut self,
        (
            settings,
            entities,
            transforms,
            harmfuls,
            harmables,
            collisions,
            mut players,
            mut enemies,
            mut velocities,
        ): Self::SystemData,
    ) {
        // TODO: Get knockback value from some component (`Knockback` component?)
        // const KNOCKBACK: (f32, f32) = (500.0, 1000.0);
        let knockback = settings.harmful.knockback_strength;

        for (entity_harmable, collision_harmable, _) in
            (&entities, &collisions, &harmables).join()
        {
            let harmable_id = entity_harmable.id();

            for (entity_harmful, harmful, transform_harmful) in
                (&entities, &harmfuls, &transforms).join()
            {
                let harmful_id = entity_harmful.id();
                if let Some(collision::Data {
                    side: Side::Inner,
                    state: collision::State::Enter,
                    ..
                }) = collision_harmable.collision_with(harmful_id)
                {
                    // Deal damage
                    deal_damage_to(
                        (
                            &entities,
                            &transforms,
                            &mut players,
                            &mut enemies,
                            &mut velocities,
                        ),
                        transform_harmful.into(),
                        harmable_id,
                        harmful.damage,
                        knockback,
                    );
                }
            }
        }
    }
}

fn deal_damage_to(
    (entities, transforms, players, enemies, velocities): (
        &Entities,
        &ReadStorage<Transform>,
        &mut WriteStorage<Player>,
        &mut WriteStorage<Enemy>,
        &mut WriteStorage<Velocity>,
    ),
    harmful_pos: Vector,
    target_id: Index,
    damage: u32,
    target_knockback: (f32, f32),
) {
    fn apply_knockback(
        kb_pos: Vector,
        target_pos: Vector,
        target_velocity: &mut Velocity,
        base_knockback: (f32, f32),
    ) {
        let knockback = (
            if target_pos.0 > kb_pos.0 {
                base_knockback.0
            } else if target_pos.0 < kb_pos.0 {
                -base_knockback.0
            } else {
                0.0
            },
            if target_pos.1 > kb_pos.1 {
                base_knockback.1
            } else if target_pos.1 < kb_pos.1 {
                -base_knockback.1
            } else {
                0.0
            },
        );
        target_velocity.x = knockback.0;
        target_velocity.y = knockback.1;
    }

    if let Some((player, transform, velocity_opt)) =
        (entities, players, transforms, velocities.maybe())
            .join()
            .find(|(entity, _, _, _)| entity.id() == target_id)
            .map(|(_, player, transform, velocity)| {
                (player, transform, velocity)
            })
    {
        player.take_damage(damage);
        if let Some(velocity) = velocity_opt {
            apply_knockback(
                harmful_pos,
                transform.into(),
                velocity,
                target_knockback,
            );
        }
    } else if let Some((enemy, transform, velocity_opt)) =
        (entities, enemies, transforms, velocities.maybe())
            .join()
            .find(|(entity, _, _, _)| entity.id() == target_id)
            .map(|(_, enemy, transform, velocity)| (enemy, transform, velocity))
    {
        enemy.take_damage(damage);
        if let Some(velocity) = velocity_opt {
            apply_knockback(
                harmful_pos,
                transform.into(),
                velocity,
                target_knockback,
            );
        }
    }
}
