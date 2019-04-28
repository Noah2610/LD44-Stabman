use super::system_prelude::*;

pub struct GoalSystem;

impl<'a> System<'a> for GoalSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Collision>,
        WriteStorage<'a, Goal>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, AnimationsContainer>,
    );

    fn run(
        &mut self,
        (
            entities,
            collisions,
            mut goals,
            mut players,
            mut animations_containers,
        ): Self::SystemData,
    ) {
        for (player, player_collision, player_animations_container) in
            (&mut players, &collisions, &mut animations_containers).join()
        {
            for (goal_entity, goal) in (&entities, &mut goals).join() {
                let goal_id = goal_entity.id();

                if !goal.next_level
                    && player_collision.in_collision_with(goal_id)
                {
                    // Beat the level
                    goal.next_level = true;
                    player.in_control = false;
                    player_animations_container.play("level_end");
                }
            }
        }
    }
}
