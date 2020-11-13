extern crate amethyst;

use amethyst::ecs::{Component, DenseVecStorage};

use crate::config;

// TODO: change this to use amethyst's animation types
// and load them from configuration files.
const PLAYER_ANIMATION_TIME: f32 = 0.5;
const PLAYER_ANIMATION_FRAMES: usize = 8;
const PLAYER_ANIMATION_FRAME_VECTOR: [usize; PLAYER_ANIMATION_FRAMES] = [2, 3, 0, 3, 2, 4, 1, 4];

pub struct Player {
    pub height: f32,
    pub width: f32,
    pub speed: f32,
    moment: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            width: config::PLAYER_WIDTH,
            height: config::PLAYER_HEIGHT,
            speed: config::DEFAULT_PLAYER_SPEED,
            moment: 0.0,
        }
    }

    pub fn add_delta(&mut self, delta: f32) {
        self.moment = (self.moment + delta / PLAYER_ANIMATION_TIME).fract()
    }

    pub fn frame(&self) -> usize {
        let scaled_moment = self.moment * (PLAYER_ANIMATION_FRAMES + 1) as f32;
        let position = scaled_moment.trunc() as usize;
        // The modulo wouldn't be necessary if not for floating point rounding errors.
        PLAYER_ANIMATION_FRAME_VECTOR[position % PLAYER_ANIMATION_FRAMES]
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
