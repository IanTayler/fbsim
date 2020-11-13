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

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Player {
    pub speed: f32,
    pub action: Option<ActionType>,
    pub kick_strength: f32,
    pub push_strength: f32,
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
