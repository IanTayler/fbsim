use crate::{
    components::player::ActionType,
    engines::{Engine, EngineData, EngineTransition},
};
use amethyst::core::math::Vector2;
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

pub struct Basic;

impl Basic {
    pub fn new() -> Self {
        Basic
    }
}

const REPOSITION_SIZE: f32 = 5.0;
const COLLISION_DISTANCE_TO_BALL: f32 = 4.0;
const CLOSE_ENOUGH_KICK: f32 = 5.0;

fn defend_with_net_weight(engine_data: EngineData, net_weight: f32) -> EngineTransition {
    let intended_position = (engine_data.own_net_position * net_weight + engine_data.ball_position)
        / (net_weight + 1.0);
    let velocity = intended_position - engine_data.own_position;
    EngineTransition {
        velocity: if velocity.norm() > 1.0 {
            (velocity / velocity.norm()) * engine_data.own.speed
        } else {
            Vector2::<f32>::new(0.0, 0.0)
        },
        action: None,
    }
}

fn forward_basic(engine_data: EngineData) -> EngineTransition {
    let ball_diff_net = engine_data.ball_position - engine_data.opponent_net_position;
    let own_diff_net = engine_data.own_position - engine_data.opponent_net_position;
    let ball_direction = engine_data.ball_position - engine_data.own_position;
    let (action, velocity) = {
        let reposition_unsized = Vector2::new(ball_direction.y, -ball_direction.x);
        let reposition = (reposition_unsized / reposition_unsized.norm())
            * (engine_data.own.speed / REPOSITION_SIZE);
        if ball_diff_net.norm() >= own_diff_net.norm() {
            (None, ball_direction + reposition)
        } else {
            // Move towards the ball and kick it!
            // Try to have the right angle to put the ball in the net.
            let ball_angle = ball_diff_net.x / ball_diff_net.y;
            let kick_x_diff_net = ball_angle
                * (ball_diff_net.y + COLLISION_DISTANCE_TO_BALL.copysign(ball_diff_net.x));
            let net_x = engine_data.opponent_net_position.x;
            let perfect_kick_x = net_x + kick_x_diff_net;
            let adjust_kick_angle = perfect_kick_x - engine_data.own_position.x;
            // Compute whether we're close to where we'd need to be to get a good kick out. Used to
            // choose whether to have a Kick action.
            let x_difference_kick =
                net_x + (ball_angle * own_diff_net.y) - engine_data.own_position.x;
            (
                if x_difference_kick.abs() < CLOSE_ENOUGH_KICK {
                    Some(ActionType::Kick)
                } else {
                    None
                },
                ball_direction + Vector2::new(adjust_kick_angle, 0.0),
            )
        }
    };
    // Add a little randomness to it, mostly to avoid deadlocks with
    // players bumping against each other.
    let noise_x: f32 = thread_rng().sample(StandardNormal);
    let noise_y: f32 = thread_rng().sample(StandardNormal);
    EngineTransition {
        velocity: (velocity / velocity.norm()) * engine_data.own.speed
            + Vector2::new(noise_x, noise_y),
        action: action,
    }
}

impl Engine for Basic {
    fn goalie(&mut self, engine_data: EngineData) -> EngineTransition {
        defend_with_net_weight(engine_data, 2.0)
    }

    fn defender(&mut self, engine_data: EngineData) -> EngineTransition {
        defend_with_net_weight(engine_data, 0.7)
    }

    fn left(&mut self, engine_data: EngineData) -> EngineTransition {
        self.forward(engine_data)
    }

    fn right(&mut self, engine_data: EngineData) -> EngineTransition {
        self.forward(engine_data)
    }

    fn forward(&mut self, engine_data: EngineData) -> EngineTransition {
        forward_basic(engine_data)
    }
}

pub struct BasicWingWait;

impl Engine for BasicWingWait {
    fn goalie(&mut self, engine_data: EngineData) -> EngineTransition {
        defend_with_net_weight(engine_data, 2.0)
    }

    fn defender(&mut self, engine_data: EngineData) -> EngineTransition {
        defend_with_net_weight(engine_data, 0.7)
    }

    fn left(&mut self, engine_data: EngineData) -> EngineTransition {
        self.forward(engine_data)
    }

    fn right(&mut self, engine_data: EngineData) -> EngineTransition {
        self.forward(engine_data)
    }

    fn forward(&mut self, engine_data: EngineData) -> EngineTransition {
        forward_basic(engine_data)
    }
}
