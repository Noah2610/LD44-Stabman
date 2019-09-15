use super::system_prelude::*;

pub struct SyncHeartsContainersWithHealthSystem;

impl<'a> System<'a> for SyncHeartsContainersWithHealthSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, HeartsContainer>,
    );

    fn run(
        &mut self,
        (entities, players, enemies, mut hearts_containers): Self::SystemData,
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
                for heart_id in hearts_container.heart_ids.iter() {
                    let heart_entity = entities.entity(*heart_id);
                    if entities.is_alive(heart_entity) {
                        entities.delete(heart_entity).unwrap();
                    }
                }
                hearts_container.heart_ids.clear();
                hearts_container.health = 0;
            } else if hearts_container.health != enemy.health {
                hearts_container.health = enemy.health;
            }
        }
    }
}
