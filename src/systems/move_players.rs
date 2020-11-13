extern crate amethyst;

use crate::components::Player;
use crate::config::*;
use crate::utils;
use amethyst::{
    core::timing::Time,
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct MovePlayers;

fn movement_multiplier(raw_movement_x: f32, raw_movement_y: f32, speed: f32) -> f32 {
    // We compute what's the right multiplier for the individual axis movement if
    // we want the euclidean distance moved in a second to be a maximum of "speed".
    // Assuming raw movements go from -1 to 1.
    let max_movement = f32::max(raw_movement_x.abs(), raw_movement_y.abs());
    let target_speed = (speed * max_movement).powi(2);
    let actual_speed = raw_movement_x.powi(2) + raw_movement_y.powi(2);
    if actual_speed == 0.0 {
        0.0
    } else {
        (target_speed / actual_speed).sqrt()
    }
}

impl<'s> System<'s> for MovePlayers {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (players, mut transforms, input, time): Self::SystemData) {
        let time_elapsed = time.delta_seconds();
        for (player, transform) in (&players, &mut transforms).join() {
            let (raw_movement_x, raw_movement_y) = utils::input_movement(&input);
            let speed = player.speed;
            let move_multiplier = movement_multiplier(raw_movement_x, raw_movement_y, speed);
            let (movement_x, movement_y) = (
                raw_movement_x * move_multiplier,
                raw_movement_y * move_multiplier,
            );
            if movement_x != 0.0 {
                let delta_x = time_elapsed * movement_x;
                let new_x = (transform.translation().x + delta_x)
                    .max(PLAYER_WIDTH / 2.0)
                    .min(SCREEN_WIDTH - PLAYER_WIDTH / 2.0);
                transform.set_translation_x(new_x);
            };
            if movement_y != 0.0 {
                let delta_y = time_elapsed * movement_y;
                let new_y = (transform.translation().y + delta_y)
                    .max(PLAYER_HEIGHT / 2.0)
                    .min(SCREEN_HEIGHT - PLAYER_HEIGHT / 2.0);
                transform.set_translation_y(new_y);
            };
        }
    }
}
