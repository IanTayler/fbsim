use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Human;

impl Component for Human {
    type Storage = DenseVecStorage<Self>;
}
