use super::system_prelude::*;

enum LoadAction {
    Load(Entity),
    Unload(Entity),
}

#[derive(Default)]
pub struct LoaderSystem;

/// Loads loadable entities when they are within the camera.
impl<'a> System<'a> for LoaderSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        Entities<'a>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Loader>,
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
            loaders,
            transforms,
            sizes,
            enemies,
            loadables,
            mut loadeds,
        ): Self::SystemData,
    ) {
        for (camera_opt, loader, loader_transform, loader_size_opt) in
            (cameras.maybe(), &loaders, &transforms, sizes.maybe()).join()
        {
            let loader_pos = {
                let pos = loader_transform.translation();
                match camera_opt.as_ref() {
                    None => (pos.x, pos.y),
                    // If the loader is the camera, then its position's origin is bottom-left,
                    // so we need to change the position we are working with accordingly.
                    Some(_) => {
                        let size = loader_size_opt.expect(
                            "The camera needs to have a size as a loader",
                        );
                        (pos.x + size.w * 0.5, pos.y + size.h * 0.5)
                    }
                }
            };
            let mut entities_to_load_or_unload: Vec<LoadAction> = Vec::new();

            for (entity, transform, size_opt, _, loaded_opt, enemy_opt) in (
                &entities,
                &transforms,
                sizes.maybe(),
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
                let size =
                    size_opt.map(|s| s.into()).unwrap_or(Vector::new(0.0, 0.0));
                let load_distance = {
                    let loader_distance = match loader.distance.as_ref() {
                        None => {
                            let loader_size = loader_size_opt.expect(
                                "Loader needs to either have its `distance` \
                                 field be Some or it needs to have a size \
                                 component",
                            );
                            (
                                loader_size.w * 0.5 + size.0 * 0.5,
                                loader_size.h * 0.5 + size.1 * 0.5,
                            )
                        }
                        Some(distance) => (distance.0, distance.1),
                    };
                    match enemy_opt {
                        None => {
                            let difference = settings
                                .entity_loader
                                .enemy_load_distance_difference;
                            (
                                loader_distance.0 + difference.0,
                                loader_distance.1 + difference.1,
                            )
                        }
                        Some(_) => loader_distance,
                    }
                };

                let pos = transform.translation();
                let distance = (
                    (loader_pos.0 - pos.x).abs(),
                    (loader_pos.1 - pos.y).abs(),
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
