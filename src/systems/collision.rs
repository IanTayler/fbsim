extern crate amethyst;

use crate::{
    components::{collision_box, Ball, CollisionBox, Player},
    config::*,
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
        WriteStorage<'s, Ball>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, CollisionBox>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (entities, mut balls, players, collision_boxes, transforms): Self::SystemData,
    ) {
        // TODO: some semblance of efficiency for all this.
        for (_ball_entity, ball, ball_collision, ball_transform) in
            (&entities, &mut balls, &collision_boxes, &transforms).join()
        {
            // First handle collisions with the end of the screen.
            {
                let ball_upper_left = ball_collision.upper_left_distance;
                let ball_lower_right = ball_collision.lower_right_distance;
                let pos = ball_transform.translation().xy();
                let vel = ball.velocity;
                let off_left = pos.x <= -ball_upper_left.x && vel.x <= 0.0;
                let off_right = pos.x >= SCREEN_WIDTH - ball_lower_right.x && vel.x >= 0.0;
                if off_left || off_right {
                    ball.velocity.x *= -1.0;
                }
                let off_bottom = pos.y <= -ball_lower_right.y && vel.y <= 0.0;
                let off_top = pos.y >= SCREEN_HEIGHT - ball_upper_left.y && vel.y >= 0.0;
                if off_bottom || off_top {
                    ball.velocity.y *= -1.0;
                }
            }
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
                    ball.velocity =
                        center_difference / center_difference.norm() * player.strength();
                }
            }
        }
    }
}
