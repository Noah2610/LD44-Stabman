use super::system_prelude::*;

enum LoadAction {
    Load(Entity),
    Unload(Entity),
}

#[derive(Default)]
pub struct LoaderSystem;

/// Loads loadable entities when they are within a certain range to the camera's center.
impl<'a> System<'a> for LoaderSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        Entities<'a>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Loadable>,
        WriteStorage<'a, Loaded>,
    );

    fn run(
        &mut self,
        (
            settings,
            entities,
            cameras,
            transforms,
            sizes,
            enemies,
            loadables,
            mut loadeds,
        ): Self::SystemData,
    ) {
        if let Some((_, camera_transform, camera_size)) =
            (&cameras, &transforms, &sizes).join().next()
        {
            let camera_pos = {
                let pos = camera_transform.translation();
                (pos.x + camera_size.w * 0.5, pos.y + camera_size.h * 0.5)
            };
            let mut entities_to_load_or_unload: Vec<LoadAction> = Vec::new();

            for (entity, transform, _, loaded_opt, enemy_opt) in (
                &entities,
                &transforms,
                &loadables,
                loadeds.maybe(),
                enemies.maybe(),
            )
                .join()
            {
                // let load_distance = match enemy_opt {
                //     None => settings.entity_loader.load_distance,
                //     Some(_) => (
                //         settings.entity_loader.load_distance.0
                //             - settings
                //                 .entity_loader
                //                 .enemy_load_distance_substraction
                //                 .0,
                //         settings.entity_loader.load_distance.1
                //             - settings
                //                 .entity_loader
                //                 .enemy_load_distance_substraction
                //                 .1,
                //     ),
                // };
                let load_distance = match enemy_opt {
                    None => {
                        let difference = settings
                            .entity_loader
                            .enemy_load_distance_difference;
                        (
                            camera_size.w * 0.5 + difference.0,
                            camera_size.h * 0.5 + difference.1,
                        )
                    }
                    Some(_) => (camera_size.w * 0.5, camera_size.h * 0.5),
                };

                let pos = transform.translation();
                let distance = (
                    (camera_pos.0 - pos.x).abs(),
                    (camera_pos.1 - pos.y).abs(),
                );

                match loaded_opt {
                    None => {
                        (if distance.0 <= load_distance.0
                            && distance.1 <= load_distance.1
                        {
                            entities_to_load_or_unload
                                .push(LoadAction::Load(entity));
                        })
                    }
                    Some(_) => {
                        (if distance.0 > load_distance.0
                            || distance.1 > load_distance.1
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
