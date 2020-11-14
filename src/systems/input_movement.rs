use crate::{
    components::{Human, MovementState, Player},
    utils,
};
use amethyst::{
    core::Transform,
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

fn rotate_player(movement_x: f32, movement_y: f32, transform: &mut Transform) {
    let has_movement = movement_x.abs() + movement_y.abs() > 0.0;
    if has_movement {
        let rotation = if movement_x.abs() >= movement_y.abs() {
            if movement_x >= 0.0 {
                std::f32::consts::PI / 2.0
            } else {
                3.0 * std::f32::consts::PI / 2.0
            }
        } else {
            if movement_y >= 0.0 {
                std::f32::consts::PI
            } else {
                0.0
            }
        };
        transform.set_rotation_2d(rotation);
    }
}

impl<'s> System<'s> for InputMovement {
    type SystemData = (
        ReadStorage<'s, Player>,
        ReadStorage<'s, Human>,
        WriteStorage<'s, MovementState>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (players, humans, mut movement_states, mut transforms, input): Self::SystemData,
    ) {
        for (player, _human, movement_state, transform) in
            (&players, &humans, &mut movement_states, &mut transforms).join()
        {
            let (raw_movement_x, raw_movement_y) = utils::input_movement(&input);
            let speed = player.speed;
            let move_multiplier = movement_multiplier(raw_movement_x, raw_movement_y, speed);
            let (movement_x, movement_y) = (
                raw_movement_x * move_multiplier,
                raw_movement_y * move_multiplier,
            );
            rotate_player(movement_x, movement_y, transform);
            movement_state.velocity.x += movement_x;
            movement_state.velocity.y += movement_y;
        }
    }
}
