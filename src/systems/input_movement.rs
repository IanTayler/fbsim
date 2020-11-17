use crate::{
    components::{Human, MovementState, Player},
    utils,
};
use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct InputMovement;

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

impl<'s> System<'s> for InputMovement {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Human>,
        WriteStorage<'s, MovementState>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (players, humans, mut movement_states, input): Self::SystemData) {
        for (player, _human, movement_state) in (&players, &humans, &mut movement_states).join() {
            let (raw_movement_x, raw_movement_y) = utils::input_movement(&input);
            let speed = player.speed;
            let move_multiplier = movement_multiplier(raw_movement_x, raw_movement_y, speed);
            let (movement_x, movement_y) = (
                raw_movement_x * move_multiplier,
                raw_movement_y * move_multiplier,
            );
            movement_state.velocity.x += movement_x;
            movement_state.velocity.y += movement_y;
        }
    }
}
