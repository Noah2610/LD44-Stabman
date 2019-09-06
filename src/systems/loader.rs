use super::system_prelude::*;

#[derive(Default)]
pub struct LoaderSystem;

/// Loads loadable entities when they are within the camera.
impl<'a> System<'a> for LoaderSystem {
    type SystemData = (
        ReadExpect<'a, Settings>,
        Entities<'a>,
        Read<'a, LoadingLevel>,
        Write<'a, World>,
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
            loading_level,
            mut world,
            cameras,
            loaders,
            transforms,
            sizes,
            enemies,
            loadables,
            mut loadeds,
        ): Self::SystemData,
    ) {
        // Don't do anything if level is loading.
        if loading_level.0 {
            return;
        }

        let mut entities_loader = EntitiesLoader::default();

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

                // if distance.0 <= load_distance.0
                //     && distance.1 <= load_distance.1
                // {
                //     entities_loader.load(entity);
                // } else {
                //     entities_loader.unload(entity);
                // }
                let in_distance = distance.0 <= load_distance.0
                    && distance.1 <= load_distance.1;
                match loaded_opt {
                    None if in_distance => {
                        entities_loader.load(entity);
                    }
                    Some(_) => {
                        if in_distance {
                            entities_loader.maintain_loaded(entity);
                        } else {
                            entities_loader.unload(entity);
                        }
                    }
                    _ => (), // Do nothing
                }
            }
        }

        // {
        //     // TODO
        //     use amethyst::core::nalgebra::Point3;
        //     use amethyst::renderer::Rgba;

        //     let mut draw_x = |pos: (f32, f32), color: Rgba| {
        //         const Z: f32 = 9.0;
        //         const LEN: f32 = 8.0;
        //         let start = Point3::new(pos.0 - LEN, pos.1 - LEN, Z);
        //         let end = Point3::new(pos.0 + LEN, pos.1 + LEN, Z);
        //         debug_lines.draw_line(start, end, color);
        //         let start = Point3::new(pos.0 - LEN, pos.1 + LEN, Z);
        //         let end = Point3::new(pos.0 + LEN, pos.1 - LEN, Z);
        //         debug_lines.draw_line(start, end, color);
        //     };

        //     for (entity, transform) in (&entities, &transforms).join() {
        //         let pos = {
        //             let trans = transform.translation();
        //             (trans.x, trans.y)
        //         };
        //         for to_load in entities_loader.to_load.iter() {
        //             if to_load.id() == entity.id() {
        //                 draw_x(pos, Rgba::GREEN);
        //             }
        //         }
        //         for to_unload in entities_loader.to_unload.iter() {
        //             if to_unload.id() == entity.id() {
        //                 draw_x(pos, Rgba::RED);
        //             }
        //         }
        //     }
        // }

        entities_loader.work(&mut loadeds, &mut world);
    }
}

#[derive(Default)]
struct EntitiesLoader {
    to_load:            Vec<Entity>,
    to_unload:          Vec<Entity>,
    to_maintain_loaded: Vec<Entity>,
}

impl EntitiesLoader {
    pub fn load(&mut self, entity: Entity) {
        if !self.to_load.contains(&entity) {
            self.to_load.push(entity);
            self.maintain_loaded(entity);
        }
    }

    pub fn unload(&mut self, entity: Entity) {
        // Only unload if it isn't already staged for loading.
        if !self.to_load.contains(&entity) && !self.to_unload.contains(&entity)
        {
            self.to_unload.push(entity);
        }
    }

    pub fn maintain_loaded(&mut self, entity: Entity) {
        if !self.to_maintain_loaded.contains(&entity) {
            self.to_maintain_loaded.push(entity);
        }
    }

    pub fn work(self, loadeds: &mut WriteStorage<Loaded>, world: &mut World) {
        for entity in self.to_unload {
            if loadeds.contains(entity) {
                if !self.to_maintain_loaded.contains(&entity) {
                    loadeds.remove(entity);
                }
            }
        }
        for entity in self.to_load {
            if !loadeds.contains(entity) {
                loadeds.insert(entity, Loaded).unwrap();
            }
        }
        world.maintain();
    }
}
