use crate::components;
use amethyst::{
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

#[derive(SystemDesc)]
pub struct InputActions;

impl<'s> System<'s> for InputActions {
    type SystemData = (
        WriteStorage<'s, components::Player>,
        ReadStorage<'s, components::Human>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut players, humans, input): Self::SystemData) {
        let mut action = None;
        if let Some(true) = input.action_is_down("kick") {
            action = Some(components::player::ActionType::Kick);
        }
        for (player, _) in (&mut players, &humans).join() {
            player.action = action;
        }
    }
}
