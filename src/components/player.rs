use crate::{config, utils};
use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ActionType {
    Kick,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Player {
    pub speed: f32,
    pub action: Option<ActionType>,
    pub kick_strength: f32,
    pub push_strength: f32,
    pub side: utils::Side,
}

impl Player {
    pub fn strength(&self) -> f32 {
        if let Some(ActionType::Kick) = self.action {
            self.kick_strength
        } else {
            self.push_strength
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub enum PlayerType {
    Goalie,
    Forward,
    Left,
    Right,
    Defender,
}

impl PlayerType {
    pub const fn from_index(i: usize) -> Option<Self> {
        match i {
            config::FORWARD_NUMBER => Some(PlayerType::Forward),
            config::GOALIE_NUMBER => Some(PlayerType::Goalie),
            config::LEFT_NUMBER => Some(PlayerType::Left),
            config::RIGHT_NUMBER => Some(PlayerType::Right),
            config::DEFENDER_NUMBER => Some(PlayerType::Defender),
            _ => None,
        }
    }

    pub const fn to_index(&self) -> usize {
        match self {
            PlayerType::Goalie => config::GOALIE_NUMBER,
            PlayerType::Forward => config::FORWARD_NUMBER,
            PlayerType::Left => config::LEFT_NUMBER,
            PlayerType::Right => config::RIGHT_NUMBER,
            PlayerType::Defender => config::DEFENDER_NUMBER,
        }
    }
}

impl Component for PlayerType {
    type Storage = DenseVecStorage<Self>;
}
