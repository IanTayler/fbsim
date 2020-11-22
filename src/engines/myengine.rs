// src/engines/myengine.rs
//
// First import everything we need.
// "crate" are imports relative to our own code.
// So crate::components is under src/components, for example.
use crate::{
    components::player::ActionType,
    engines::{EngineData, EngineTransition, SimpleEngine},
};
use amethyst::core::math;

// We don't need any fields. We just define an empty struct.
pub struct MyEngine;

// We use `impl` to implement the `SimpleEngine` trait.
impl SimpleEngine for MyEngine {
    fn engine_func(&mut self, engine_data: EngineData) -> EngineTransition {
        // In Rust, the last expression in the function is the return value!
        // Here, we return an EngineTransition, as the trait defines.
        // Remember not to add a semicolon (;) for the return expression
        // as that evaluates to an empty value!
        EngineTransition {
            // Stay put! Moving is dangerous!!!! (´･_･`)
            velocity: math::Vector2::new(0.0, 0.0),
            // Don't act! Actions have consequences!!!!! (´･_･`)
            action: None,
        }
    }
}
