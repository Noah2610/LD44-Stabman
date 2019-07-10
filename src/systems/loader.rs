use super::system_prelude::*;

// TODO: Temporary
const LOAD_DISTANCE: (f32, f32) = (512.0, 512.0);

enum LoadAction {
    Load(Entity),
    Unload(Entity),
}

#[derive(Default)]
pub struct LoaderSystem;

/// Loads loadable entities when they are within a certain range to the player.
impl<'a> System<'a> for LoaderSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Loadable>,
        WriteStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (entities, players, transforms, loadables, mut loadeds): Self::SystemData,
    ) {
        if let Some((_, player_transform)) =
            (&players, &transforms).join().next()
        {
            let player_pos = player_transform.translation();
            let mut entities_to_load_or_unload: Vec<LoadAction> = Vec::new();

            for (entity, transform, _, loaded_opt) in
                (&entities, &transforms, &loadables, loadeds.maybe()).join()
            {
                let pos = transform.translation();
                let distance = (
                    (player_pos.x - pos.x).abs(),
                    (player_pos.y - pos.y).abs(),
                );

                match loaded_opt {
                    None => {
                        (if distance.0 <= LOAD_DISTANCE.0
                            && distance.1 <= LOAD_DISTANCE.1
                        {
                            entities_to_load_or_unload
                                .push(LoadAction::Load(entity));
                        })
                    }
                    Some(_) => {
                        (if distance.0 > LOAD_DISTANCE.0
                            || distance.1 > LOAD_DISTANCE.1
                        {
                            entities_to_load_or_unload
                                .push(LoadAction::Unload(entity));
                        })
                    }
                }
            }

            for load_action in entities_to_load_or_unload {
                match load_action {
                    LoadAction::Load(entity) => {
                        loadeds.insert(entity, Loaded).unwrap();
                    }
                    LoadAction::Unload(entity) => {
                        loadeds.remove(entity);
                    }
                }
            }
        }
    }
}
