use crate::{
    components::{collision_box, Ball, CollisionBox, MovementState, Player},
    config::*,
    rectangle::Rectangle,
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct Collisions;

impl<'s> System<'s> for Collisions {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, MovementState>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, CollisionBox>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (entities, mut movement_states, balls, players, collision_boxes, transforms): Self::SystemData,
    ) {
        // TODO: some semblance of efficiency for all this.
        // Handle collision that apply to all entities with a CollisionBox.
        for (entity, movement_state, collision, transform) in (
            &entities,
            &mut movement_states,
            &collision_boxes,
            &transforms,
        )
            .join()
        {
            // First handle collisions with the end of the screen.
            let upper_left = collision.upper_left_distance;
            let lower_right = collision.lower_right_distance;
            let pos = transform.translation().xy();
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
                    // Ignore the case where we have ball vs. player
                    match (balls.get(entity), players.get(other_entity)) {
                        (Some(_), Some(_)) => continue,
                        _ => {
                            let box_center = collision.rectangle(transform).center();
                            let other_box_center =
                                other_collision.rectangle(other_transform).center();
                            let center_difference = box_center - other_box_center;
                            movement_state.velocity = center_difference / center_difference.norm()
                                * movement_state.velocity.norm();
                        }
                    }
                }
            }
        }
        // These handling collisions with balls, specifically.
        for (movement_state, _ball, ball_collision, ball_transform) in
            (&mut movement_states, &balls, &collision_boxes, &transforms).join()
        {
            // Now handle collisions with players (i.e. kicks)
            for (_player_entity, player, player_collision, player_transform) in
                (&entities, &players, &collision_boxes, &transforms).join()
            {
                if collision_box::are_colliding(
                    ball_collision,
                    ball_transform,
                    player_collision,
                    player_transform,
                ) {
                    let center_difference =
                        ball_transform.translation().xy() - player_transform.translation().xy();
                    movement_state.velocity =
                        center_difference / center_difference.norm() * player.strength();
                }
            }
        }
    }
}
