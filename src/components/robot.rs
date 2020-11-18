use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum LogicModule {
    SillyRun,
    EngineRunner(String),
}
#[derive(Clone, Debug, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
pub struct Robot {
    pub logic_module: LogicModule,
}

impl Default for Robot {
    fn default() -> Self {
        Robot {
            logic_module: LogicModule::EngineRunner("basic".to_string()),
        }
    }
}

impl Component for Robot {
    type Storage = DenseVecStorage<Self>;
}
