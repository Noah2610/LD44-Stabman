use super::system_prelude::*;

pub struct SyncHeartsContainersWithHealthSystem;

impl<'a> System<'a> for SyncHeartsContainersWithHealthSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, HeartsContainer>,
        WriteStorage<'a, Heart>,
    );

    fn run(
        &mut self,
        (entities, players, enemies, mut hearts_containers, mut hearts): Self::SystemData,
    ) {
        for (player, mut hearts_container) in
            (&players, &mut hearts_containers).join()
        {
            if hearts_container.health != player.health {
                hearts_container.health = player.health;
            }
        }

        for (enemy, mut hearts_container) in
            (&enemies, &mut hearts_containers).join()
        {
            if enemy.health == 0 {
                for id in hearts_container.heart_ids.iter() {
                    entities.delete(entities.entity(*id)).unwrap();
                }
            } else if hearts_container.health != enemy.health {
                hearts_container.health = enemy.health;
            }
        }
    }
}
