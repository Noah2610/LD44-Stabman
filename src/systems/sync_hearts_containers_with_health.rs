use super::system_prelude::*;

pub struct SyncHeartsContainersWithHealthSystem;

impl<'a> System<'a> for SyncHeartsContainersWithHealthSystem {
    type SystemData = (
        ReadStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        WriteStorage<'a, HeartsContainer>,
    );

    fn run(
        &mut self,
        (players, enemies, mut hearts_containers): Self::SystemData,
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
            if hearts_container.health != enemy.health {
                hearts_container.health = enemy.health;
            }
        }
    }
}
