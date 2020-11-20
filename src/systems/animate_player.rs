use crate::components::{MovementState, Player};
use crate::fbsim::AnimationId;
use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData, WriteStorage},
    renderer::SpriteRender,
};

const MIN_MOVEMENT: f32 = 1.0;

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

#[derive(SystemDesc)]
pub struct AnimatePlayer;

impl<'s> System<'s> for AnimatePlayer {
    type SystemData = (
        Entities<'s>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, AnimationSet<AnimationId, SpriteRender>>,
        ReadStorage<'s, MovementState>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, AnimationControlSet<AnimationId, SpriteRender>>,
    );

    fn run(
        &mut self,
        (entities, players, animation_sets, movement_states,mut transforms, mut control_sets): Self::SystemData,
    ) {
        for (entity, _player, movement_state, animation_set, transform) in (
            &entities,
            &players,
            &movement_states,
            &animation_sets,
            &mut transforms,
        )
            .join()
        {
            // Creates a new AnimationControlSet for the entity
            let control_set = get_animation_set(&mut control_sets, entity).unwrap();
            let velocity = movement_state.velocity;
            if velocity.x < MIN_MOVEMENT && velocity.y < MIN_MOVEMENT {
                control_set.abort(AnimationId::PlayerRun);
                control_set.add_animation(
                    AnimationId::PlayerStand,
                    &animation_set.get(&AnimationId::PlayerStand).unwrap(),
                    EndControl::Normal,
                    1.0,
                    AnimationCommand::Start,
                );
            } else {
                rotate_player(velocity.x, velocity.y, transform);
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
