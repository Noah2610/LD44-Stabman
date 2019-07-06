use super::system_prelude::*;

pub struct HarmfulSystem;

impl<'a> System<'a> for HarmfulSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Harmful>,
        ReadStorage<'a, Harmable>,
        ReadStorage<'a, Collision>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Enemy>,
    );

    fn run(
        &mut self,
        (entities, harmfuls, harmables, collisions, mut players, mut enemies): Self::SystemData,
    ) {
        // TODO: Get knockback value from some component (`Knockback` component?)
        // const KNOCKBACK: (f32, f32) = (500.0, 1000.0);

        for (entity_harmable, collision_harmable, _) in
            (&entities, &collisions, &harmables).join()
        {
            let harmable_id = entity_harmable.id();

            for (entity_harmful, harmful) in (&entities, &harmfuls).join() {
                let harmful_id = entity_harmful.id();
                if let Some(collision::Data {
                    side: Side::Inner,
                    state: collision::State::Enter,
                    ..
                }) = collision_harmable.collision_with(harmful_id)
                {
                    // Deal damage
                    deal_damage_to(
                        (&entities, &mut players, &mut enemies),
                        harmable_id,
                        harmful.damage,
                    );
                }
            }
        }
    }
}

fn deal_damage_to(
    (entities, players, enemies): (
        &Entities,
        &mut WriteStorage<Player>,
        &mut WriteStorage<Enemy>,
    ),
    target_id: Index,
    damage: u32,
) {
    if let Some(player) = (entities, players)
        .join()
        .find(|(entity, _)| entity.id() == target_id)
        .map(|(_, player)| player)
    {
        player.take_damage(damage);
    } else if let Some(enemy) = (entities, enemies)
        .join()
        .find(|(entity, _)| entity.id() == target_id)
        .map(|(_, enemy)| enemy)
    {
        enemy.take_damage(damage);
    }
}
