// src/engines/myengine.rs
//
// First import everything we need.
// "crate" are imports relative to our own code.
// So crate::components is under src/components, for example.
use crate::{
    components::player::ActionType,
    engines::{EngineData, EngineTransition, SimpleEngine},
};

// We don't need any fields. We just define an empty struct.
pub struct MyEngine;

// We use `impl` to implement the `SimpleEngine` trait.
impl SimpleEngine for MyEngine {
    fn engine_func(&mut self, engine_data: EngineData) -> EngineTransition {
        // The idea behind this engine is simple:
        // 1. If we're close to the ball, run towards it and kick it!
        // 2. If we're far away from the ball, run to your own net! Defense!
        //
        // First compute what's the difference in position with the ball.
        let difference_with_ball = engine_data.ball_position - engine_data.own_position;
        // Vector2 from nalgebra implements `norm()` which computes the euclidean norm.
        if difference_with_ball.norm() > 50.0 {
            // We're far away from the ball. Find in which direction is your net!
            let difference_with_own_net = engine_data.own_net_position - engine_data.own_position;
            // Now run in that direction as fast as you can!
            EngineTransition {
                velocity: difference_with_own_net * engine_data.own.speed,
                action: None,
            }
        } else {
            // We're close to the ball!
            // ATTAAAAACKKKK!!!!!
            EngineTransition {
                velocity: difference_with_ball * engine_data.own.speed,
                action: Some(ActionType::Kick),
            }
        }
    }
}
