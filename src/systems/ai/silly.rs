use crate::{components, components::robot::LogicModule, utils};
use amethyst::{
    core::{math, Transform},
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct SillyRun;

impl<'s> System<'s> for SillyRun {
    type SystemData = (
        WriteStorage<'s, components::Player>,
        ReadStorage<'s, components::Ball>,
        ReadStorage<'s, components::Robot>,
        ReadStorage<'s, components::Human>,
        ReadStorage<'s, components::Net>,
        WriteStorage<'s, components::MovementState>,
        ReadStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (mut players, balls, robots, _humans, nets, mut movement_states, transforms): Self::SystemData,
    ) {
        for (player, movement_state, player_transform, robot) in
            (&mut players, &mut movement_states, &transforms, &robots).join()
        {
            if robot.logic_module == LogicModule::SillyRun {
                let mut ball_position = math::Vector2::<f32>::new(0.0, 0.0);
                let mut velocity = math::Vector2::<f32>::new(0.0, 0.0);
                for (_, ball_transform) in (&balls, &transforms).join() {
                    ball_position += ball_transform.translation().xy();
                }
                for (net, net_transform) in (&nets, &transforms).join() {
                    if net.side == utils::Side::LowerSide {
                        let net_position = net_transform.translation().xy();
                        let player_position = player_transform.translation().xy();
                        velocity = ((ball_position + net_position) / 2.0) - player_position;
                    }
                }
                if velocity.norm() > 1.0 {
                    movement_state.velocity = (velocity / velocity.norm()) * player.speed;
                } else {
                    movement_state.velocity *= 0.0;
                }
            }
        }
    }
}
