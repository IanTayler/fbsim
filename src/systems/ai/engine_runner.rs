use crate::{
    components, components::robot::LogicModule, engines::EngineData, resources::EngineRegistry,
    utils,
};
use amethyst::{
    core::{math::Vector2, Transform},
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, Write, WriteStorage},
};
use std::collections::BTreeMap;

#[derive(SystemDesc)]
pub struct EngineRunner;

impl<'s> System<'s> for EngineRunner {
    type SystemData = (
        WriteStorage<'s, components::Player>,
        ReadStorage<'s, components::PlayerType>,
        ReadStorage<'s, components::Ball>,
        ReadStorage<'s, components::Robot>,
        ReadStorage<'s, components::Human>,
        ReadStorage<'s, components::Net>,
        WriteStorage<'s, components::MovementState>,
        ReadStorage<'s, Transform>,
        Write<'s, EngineRegistry>,
    );

    fn run(
        &mut self,
        (
            mut players,
            player_types,
            balls,
            robots,
            _humans,
            nets,
            mut movement_states,
            transforms,
            mut engine_registry,
        ): Self::SystemData,
    ) {
        let mut team_position_vectors =
            BTreeMap::<utils::Side, Vec<(components::PlayerType, Vector2<f32>)>>::new();
        team_position_vectors.insert(utils::Side::UpperSide, Vec::new());
        team_position_vectors.insert(utils::Side::LowerSide, Vec::new());
        for (player, player_type, player_transform) in (&players, &player_types, &transforms).join()
        {
            team_position_vectors
                .get_mut(&player.side)
                .unwrap()
                .push((*player_type, player_transform.translation().xy()));
        }
        for (player, player_type, movement_state, player_transform, robot) in (
            &mut players,
            &player_types,
            &mut movement_states,
            &transforms,
            &robots,
        )
            .join()
        {
            if let LogicModule::EngineRunner(engine_name) = &robot.logic_module {
                let mut ball_position = Vector2::<f32>::new(0.0, 0.0);
                let mut own_net_position = Vector2::<f32>::new(0.0, 0.0);
                let mut opponent_net_position = Vector2::<f32>::new(0.0, 0.0);
                for (_, ball_transform) in (&balls, &transforms).join() {
                    ball_position += ball_transform.translation().xy();
                }
                for (net, net_transform) in (&nets, &transforms).join() {
                    if net.side == player.side {
                        own_net_position = net_transform.translation().xy();
                    } else {
                        opponent_net_position = net_transform.translation().xy();
                    }
                }
                let teammates_position_info = team_position_vectors.get(&player.side).unwrap();
                let mut teammates_position = Vec::new();
                for (p_type, pos) in teammates_position_info {
                    if p_type != player_type {
                        teammates_position.push(*pos);
                    }
                }
                let opponents_position_info = team_position_vectors
                    .get(&utils::opposite_side(player.side))
                    .unwrap();
                let mut opponents_position = Vec::new();
                for (_, pos) in opponents_position_info {
                    opponents_position.push(*pos);
                }
                let engine_data = EngineData {
                    ball_position: ball_position,
                    own: player,
                    own_type: player_type,
                    own_position: player_transform.translation().xy(),
                    own_net_position: own_net_position,
                    opponent_net_position: opponent_net_position,
                    teammates_position: teammates_position,
                    opponents_position: opponents_position,
                };
                let engine = engine_registry.get(engine_name.to_string()).unwrap();
                let engine_transition = engine.dispatch(engine_data);
                let velocity = engine_transition.velocity;
                let action = engine_transition.action;
                player.action = action;
                if velocity.norm() > player.speed {
                    movement_state.velocity = (velocity / velocity.norm()) * player.speed;
                } else {
                    movement_state.velocity = velocity;
                }
            }
        }
    }
}
