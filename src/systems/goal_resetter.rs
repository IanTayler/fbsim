use crate::{
    components::{Ball, MovementState, Player, PlayerType},
    config::*,
    resources::Score,
    utils::player_position,
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct GoalResetter;

impl<'s> System<'s> for GoalResetter {
    type SystemData = (
        WriteStorage<'s, MovementState>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, PlayerType>,
        ReadStorage<'s, Ball>,
        Read<'s, Score>,
    );

    fn run(
        &mut self,
        (mut movement_states, mut transforms, players, player_types, balls, score): Self::SystemData,
    ) {
        if !score.in_goal_last_frame {
            return ();
        }
        for (movement_state, transform, player, player_type) in (
            &mut movement_states,
            &mut transforms,
            &players,
            &player_types,
        )
            .join()
        {
            let default_position = player_position(player_type.to_index(), player.side);
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
