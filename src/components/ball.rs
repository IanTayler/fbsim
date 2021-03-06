use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Ball;

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}
