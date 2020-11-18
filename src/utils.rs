use crate::config;
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

/// Return the opposite side (Upper if Lower, and viceversa)
pub fn opposite_side(side: Side) -> Side {
    match side {
        Side::UpperSide => Side::LowerSide,
        Side::LowerSide => Side::UpperSide,
    }
}

/// Return the default position for player number i.
pub fn player_position(i: usize, side: Side) -> [f32; 2] {
    let mut position = config::RESET_PLAYER_POSITIONS[i];
    if side == Side::UpperSide {
        position[1] = config::SCREEN_HEIGHT - position[1];
    }
    position
}
