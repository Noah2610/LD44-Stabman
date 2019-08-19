use super::system_prelude::*;

#[derive(Default)]
pub struct NoclipSystem;

impl<'a> System<'a> for NoclipSystem {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Settings>,
        Read<'a, Time>,
        Read<'a, InputManager>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Noclip>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, DecreaseVelocity>,
        WriteStorage<'a, Gravity>,
        WriteStorage<'a, Solid<SolidTag>>,
        WriteStorage<'a, Invincible>,
    );

    fn run(
        &mut self,
        (
            entities,
            settings,
            time,
            input_manager,
            mut players,
            mut noclips,
            mut velocities,
            mut decrease_velocities,
            mut gravities,
            mut solids,
            mut invincibles,
        ): Self::SystemData,
    ) {
        if let Some((
            player_entity,
            player,
            mut player_velocity,
            mut player_decrease_velocity_opt,
            mut player_solid_opt,
        )) = (
            &entities,
            &mut players,
            &mut velocities,
            (&mut decrease_velocities).maybe(),
            (&mut solids).maybe(),
        )
            .join()
            .next()
        {
            let mut is_noclip = noclips.contains(player_entity);

            // Toggle noclip
            if input_manager.is_up("noclip_toggle") {
                if is_noclip {
                    // DISABLE NOCLIP
                    eprintln!("noclip: DISABLED");
                    is_noclip = false;
                    noclips.remove(player_entity).unwrap();
                    if !gravities.contains(player_entity) {
                        gravities
                            .insert(
                                player_entity,
                                Gravity::from(player.gravity),
                            )
                            .unwrap();
                    }
                    if let Some(solid) =
                        player_solid_opt.as_mut().and_then(|solid| {
                            if let SolidTag::Player = solid.tag {
                                None
                            } else {
                                Some(solid)
                            }
                        })
                    {
                        solid.tag = SolidTag::Player;
                    }
                    if invincibles.contains(player_entity) {
                        invincibles.remove(player_entity).unwrap();
                    }
                } else {
                    // ENABLE NOCLIP
                    eprintln!("noclip: ENABLED");
                    is_noclip = true;
                    noclips.insert(player_entity, Noclip::default()).unwrap();
                    if gravities.contains(player_entity) {
                        gravities.remove(player_entity).unwrap();
                    }
                    if let Some(solid) =
                        player_solid_opt.as_mut().and_then(|solid| {
                            if let SolidTag::Noclip = solid.tag {
                                None
                            } else {
                                Some(solid)
                            }
                        })
                    {
                        solid.tag = SolidTag::Noclip;
                    }
                    invincibles
                        .insert(player_entity, Invincible::default())
                        .unwrap();
                }
            }

            // Move in noclip mode
            if is_noclip {
                let dt = time.delta_seconds();
                let acceleration = settings.noclip.acceleration;
                let max_velocity = settings.noclip.max_velocity;

                // Always reset all special jumps (wall jumps, dashes)
                player.reset_jumps();

                // Move on x axis
                if let Some(x) =
                    input_manager.axis_value("noclip_x").map(|x| x as f32)
                {
                    if x > 0.0 || x < 0.0 {
                        let increase = acceleration.0 * x * dt;
                        player_velocity
                            .increase_x_with_max(increase, max_velocity.0);
                        player_decrease_velocity_opt.as_mut().map(|decr| {
                            if increase > 0.0 {
                                decr.dont_decrease_x_when_pos();
                            } else if increase < 0.0 {
                                decr.dont_decrease_x_when_neg();
                            }
                        });
                    } else {
                        player_velocity.x = 0.0;
                    }
                } else {
                    eprintln!(
                        "WARNING: `noclip_x` axis is not defined in \
                         bindings.ron"
                    );
                }

                // Move on y axis
                if let Some(y) =
                    input_manager.axis_value("noclip_y").map(|y| y as f32)
                {
                    if y > 0.0 || y < 0.0 {
                        let increase = acceleration.1 * y * dt;
                        player_velocity
                            .increase_y_with_max(increase, max_velocity.1);
                        player_decrease_velocity_opt.as_mut().map(|decr| {
                            if increase > 0.0 {
                                decr.dont_decrease_y_when_pos();
                            } else if increase < 0.0 {
                                decr.dont_decrease_y_when_neg();
                            }
                        });
                    } else {
                        player_velocity.y = 0.0;
                    }
                } else {
                    eprintln!(
                        "WARNING: `noclip_y` axis is not defined in \
                         bindings.ron"
                    );
                }
            }
        }
    }
}
