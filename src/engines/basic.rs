use crate::engines::{EngineData, EngineTransition, SimpleEngine};

pub struct Basic;

impl Basic {
    pub fn new() -> Self {
        Basic
    }
}

impl SimpleEngine for Basic {
    fn engine_func(&mut self, engine_data: EngineData) -> EngineTransition {
        let intended_position = (engine_data.own_net_position + engine_data.ball_position) / 2.0;
        let velocity = intended_position - engine_data.own_position;
        EngineTransition {
            velocity: velocity,
            action: None,
        }
    }
}
