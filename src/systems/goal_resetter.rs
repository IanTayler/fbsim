use crate::{
    components::{Ball, MovementState, Player},
    config::*,
    resources::Score,
    utils::Side,
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct GoalResetter;

const MAXIMUM_TEAM_SIZE: usize = 5;
const RESET_PLAYER_POSITIONS: [[f32; 2]; MAXIMUM_TEAM_SIZE] = [
    [SCREEN_WIDTH / 2.0, 4.0 * SCREEN_HEIGHT / 9.0],
    [SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 16.0],
    [SCREEN_WIDTH / 4.0, SCREEN_HEIGHT / 3.0],
    [3.0 * SCREEN_WIDTH / 4.0, SCREEN_HEIGHT / 3.0],
    [SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 4.0],
];

impl<'s> System<'s> for GoalResetter {
    type SystemData = (
        WriteStorage<'s, MovementState>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Ball>,
        Read<'s, Score>,
    );

    fn run(
        &mut self,
        (mut movement_states, mut transforms, players, balls, score): Self::SystemData,
    ) {
        if !score.in_goal_last_frame {
            return ();
        }
        let mut team_sizes = std::collections::BTreeMap::<Side, usize>::new();
        for (movement_state, transform, player) in
            (&mut movement_states, &mut transforms, &players).join()
        {
            let mut default_position = match team_sizes.get(&player.side) {
                Some(v) => {
                    if *v < MAXIMUM_TEAM_SIZE {
                        RESET_PLAYER_POSITIONS[*v]
                    } else {
                        [0.0, 0.0]
                    }
                }
                None => RESET_PLAYER_POSITIONS[0],
            };
            team_sizes.insert(player.side, team_sizes.get(&player.side).unwrap_or(&0) + 1);
            if player.side == Side::UpperSide {
                default_position[1] = SCREEN_HEIGHT - default_position[1];
            }
            transform.set_translation_xyz(
                default_position[0],
                default_position[1],
                transform.translation().z,
            );
            movement_state.velocity *= 0.0;
        }
        for (_ball, movement_state, transform) in
            (&balls, &mut movement_states, &mut transforms).join()
        {
            movement_state.velocity *= 0.0;
            transform.set_translation_xyz(
                SCREEN_WIDTH / 2.0,
                SCREEN_HEIGHT / 2.0,
                transform.translation().z,
            );
        }
    }
}
