use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

use crate::config;

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Player {
    pub height: f32,
    pub width: f32,
    pub speed: f32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            width: config::PLAYER_WIDTH,
            height: config::PLAYER_HEIGHT,
            speed: config::DEFAULT_PLAYER_SPEED,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
