use crate::{
    components::{collision_box, Ball, CollisionBox, MovementState, Net, Player},
    config::*,
    rectangle::Rectangle,
};
use amethyst::{
    core::{math, timing::Time, Transform},
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MoveObjects;

impl<'s> System<'s> for MoveObjects {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, MovementState>,
        ReadStorage<'s, CollisionBox>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Net>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut movement_states,
            collision_boxes,
            mut transforms,
            players,
            balls,
            nets,
            time,
        ): Self::SystemData,
    ) {
        let delta_seconds = time.delta_seconds();
        for (entity, movement_state, collision, transform) in (
            &entities,
            &mut movement_states,
            &collision_boxes,
            &transforms,
        )
            .join()
        {
            // Handle collision that apply to all entities with a CollisionBox.
            // First handle collisions with the end of the screen.
            let upper_left = collision.upper_left_distance;
            let lower_right = collision.lower_right_distance;
            let pos = transform.translation().xy() + movement_state.velocity * delta_seconds;
            let vel = movement_state.velocity;
            let off_left = pos.x <= -upper_left.x && vel.x <= 0.0;
            let off_right = pos.x >= SCREEN_WIDTH - lower_right.x && vel.x >= 0.0;
            if off_left || off_right {
                movement_state.velocity.x *= -1.0;
            }
            let off_bottom = pos.y <= -lower_right.y && vel.y <= 0.0;
            let off_top = pos.y >= SCREEN_HEIGHT - upper_left.y && vel.y >= 0.0;
            if off_bottom || off_top {
                movement_state.velocity.y *= -1.0;
            }
            // Now let's handle any two pairs of collisions.
            for (other_entity, other_collision, other_transform) in
                (&entities, &collision_boxes, &transforms).join()
            {
                if entity.id() != other_entity.id()
                    && collision_box::are_colliding(
                        collision,
                        transform,
                        other_collision,
                        other_transform,
                    )
                {
                    // Ignore the case where we have ball vs. player or ball vs. net.
                    if balls.get(entity).is_none()
                        || (players.get(other_entity).is_none() && nets.get(other_entity).is_none())
                    {
                        let box_center = collision.rectangle(transform).center();
                        let other_box_center = other_collision.rectangle(other_transform).center();
                        let center_difference = box_center - other_box_center;
                        movement_state.velocity = center_difference / center_difference.norm()
                            * movement_state.velocity.norm();
                    }
                }
            }
        }
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
