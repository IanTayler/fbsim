use crate::components::MovementState;
use amethyst::{
    core::{math, timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MoveObjects;

impl<'s> System<'s> for MoveObjects {
    type SystemData = (
        WriteStorage<'s, MovementState>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut movement_states, mut transforms, time): Self::SystemData) {
        let delta_seconds = time.delta_seconds();
        for (movement_state, transform) in (&mut movement_states, &mut transforms).join() {
            let movement_norm = movement_state.velocity.norm();
            if movement_norm > 0.0 {
                transform.prepend_translation(math::Vector3::new(
                    movement_state.velocity.x * delta_seconds,
                    movement_state.velocity.y * delta_seconds,
                    0.0,
                ));
                if movement_state.reset {
                    movement_state.velocity *= 0.0;
                } else {
                    let deaccel = movement_state.deaccel * time.delta_seconds();
                    if deaccel >= movement_norm {
                        movement_state.velocity *= 0.0;
                    } else {
                        movement_state.velocity -= (movement_state.velocity
                            / movement_state.velocity.norm())
                            * movement_state.deaccel
                            * delta_seconds;
                    }
                }
            }
        }
    }
}
