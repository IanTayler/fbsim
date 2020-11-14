use amethyst::{
    assets::PrefabData,
    core::math,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

/// Added to entities that move.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct MovementState {
    /// Current velocity.
    pub velocity: math::Vector2<f32>,
    /// Deacceleration in every coordinate every tenth of a second.
    pub deaccel: f32,
    /// Whether to reset velocity every frame.
    pub reset: bool,
}

impl Component for MovementState {
    type Storage = DenseVecStorage<Self>;
}
