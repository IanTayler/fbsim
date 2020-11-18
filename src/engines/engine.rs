use crate::components::{player, player::PlayerType};
use amethyst::core::math::Vector2;

/// Input for engine functions.
pub struct EngineData<'a> {
    pub ball_position: Vector2<f32>,
    pub own_position: Vector2<f32>,
    pub own: &'a player::Player,
    pub own_type: &'a player::PlayerType,
    pub own_net_position: Vector2<f32>,
    pub opponent_net_position: Vector2<f32>,
    pub teammates_position: Vec<Vector2<f32>>,
    pub opponents_position: Vec<Vector2<f32>>,
}

/// Return value of engine functions.
pub struct EngineTransition {
    /// Velocity vector (pixels per second). Magnitude will be cropped to player speed!
    pub velocity: Vector2<f32>,
    /// Which action to activate (if any).
    pub action: Option<player::ActionType>,
}

pub trait Engine {
    fn goalie(&mut self, engine_data: EngineData) -> EngineTransition;
    fn forward(&mut self, engine_data: EngineData) -> EngineTransition;
    fn left(&mut self, engine_data: EngineData) -> EngineTransition;
    fn defender(&mut self, engine_data: EngineData) -> EngineTransition;
    fn right(&mut self, engine_data: EngineData) -> EngineTransition;
    fn dispatch(&mut self, engine_data: EngineData) -> EngineTransition {
        match engine_data.own_type {
            PlayerType::Goalie => self.goalie(engine_data),
            PlayerType::Defender => self.defender(engine_data),
            PlayerType::Forward => self.forward(engine_data),
            PlayerType::Left => self.left(engine_data),
            PlayerType::Right => self.right(engine_data),
        }
    }
}

pub trait SimpleEngine {
    fn engine_func(&mut self, engine_data: EngineData) -> EngineTransition;
}

impl<T: SimpleEngine> Engine for T {
    fn goalie(&mut self, engine_data: EngineData) -> EngineTransition {
        self.engine_func(engine_data)
    }
    fn forward(&mut self, engine_data: EngineData) -> EngineTransition {
        self.engine_func(engine_data)
    }
    fn defender(&mut self, engine_data: EngineData) -> EngineTransition {
        self.engine_func(engine_data)
    }
    fn left(&mut self, engine_data: EngineData) -> EngineTransition {
        self.engine_func(engine_data)
    }
    fn right(&mut self, engine_data: EngineData) -> EngineTransition {
        self.engine_func(engine_data)
    }
}
