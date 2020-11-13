use crate::components::{player, Player};
use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct InputActions;

impl<'s> System<'s> for InputActions {
    type SystemData = (
        WriteStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut players, input): Self::SystemData) {
        if let Some(true) = input.action_is_down("kick") {
            for player in (&mut players).join() {
                player.action = Some(player::ActionType::Kick);
            }
        } else {
            for player in (&mut players).join() {
                player.action = None;
            }
        }
    }
}
