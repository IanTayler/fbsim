use crate::components::{MovementState, Player};
use crate::fbsim::AnimationId;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
    renderer::SpriteRender,
};

#[derive(SystemDesc)]
pub struct AnimatePlayer;

impl<'s> System<'s> for AnimatePlayer {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
        ReadStorage<'s, MovementState>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(
        &mut self,
        (entities, players, animation_sets, movement_states, mut control_sets): Self::SystemData,
    ) {
        for (entity, _player, movement_state, animation_set) in
            (&entities, &players, &movement_states, &animation_sets).join()
        {
            // Creates a new AnimationControlSet for the entity
            let control_set = get_animation_set(&mut control_sets, entity).unwrap();
            let velocity = movement_state.velocity;
            if velocity.x == 0.0 && velocity.y == 0.0 {
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
