use crate::{components::Ball, config::*};
use amethyst::{
    core::{math, Transform},
    derive::SystemDesc,
    ecs::{Join, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct MoveBalls;

impl<'s> System<'s> for MoveBalls {
    type SystemData = (WriteStorage<'s, Ball>, WriteStorage<'s, Transform>);

    fn run(&mut self, (mut balls, mut transforms): Self::SystemData) {
        for (ball, transform) in (&mut balls, &mut transforms).join() {
            transform.prepend_translation(math::Vector3::new(
                ball.velocity.x,
                ball.velocity.y,
                0.0,
            ));
            if ball.velocity.norm() <= MINIMUM_BALL_VELOCITY {
                ball.velocity *= 0.0;
            } else {
                ball.velocity *= ball.deaccel;
            };
        }
    }
}
