use amethyst::input::{InputHandler, StringBindings};
use serde::{Deserialize, Serialize};

pub fn input_movement(input: &InputHandler<StringBindings>) -> (f32, f32) {
    let raw_movement_y = match input.axis_value("player_vertical") {
        Some(mov) => mov,
        None => 0.0,
    };
    let raw_movement_x = match input.axis_value("player_horizontal") {
        Some(mov) => mov,
        None => 0.0,
    };
    (raw_movement_x, raw_movement_y)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum Side {
    UpperSide,
    LowerSide,
}
