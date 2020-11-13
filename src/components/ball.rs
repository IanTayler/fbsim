use amethyst::{
    assets::PrefabData,
    core::math,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Ball {
    /// Current velocity.
    pub velocity: math::Vector2<f32>,
    /// Deacceleration factor for the ball.
    pub deaccel: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
