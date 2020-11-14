extern crate amethyst;

use crate::components::Player;
use crate::fbsim::AnimationId;
use crate::utils;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    derive::SystemDesc,
    ecs::{Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
};

#[derive(SystemDesc)]
pub struct AnimatePlayer;

impl<'s> System<'s> for AnimatePlayer {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(
        &mut self,
        (entities, players, animation_sets, mut control_sets, input): Self::SystemData,
    ) {
        for (entity, _player, animation_set) in (&entities, &players, &animation_sets).join() {
            // Creates a new AnimationControlSet for the entity
            let control_set = get_animation_set(&mut control_sets, entity).unwrap();
            let (raw_movement_x, raw_movement_y) = utils::input_movement(&input);
            if raw_movement_x == 0.0 && raw_movement_y == 0.0 {
                control_set.abort(AnimationId::PlayerRun);
                control_set.add_animation(
                    AnimationId::PlayerStand,
                    &animation_set.get(&AnimationId::PlayerStand).unwrap(),
                    EndControl::Normal,
                    1.0,
                    AnimationCommand::Start,
                );
            } else {
                control_set.abort(AnimationId::PlayerStand);
                control_set.add_animation(
                    AnimationId::PlayerRun,
                    &animation_set.get(&AnimationId::PlayerRun).unwrap(),
                    EndControl::Normal,
                    1.0,
                    AnimationCommand::Start,
                );
            };
        }
    }
}
