use crate::{
    components::{
        collision_box, player, Ball, CollisionBox, MovementState, Net, Player, PlayerType,
    },
    resources::Score,
    utils::Side,
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, Write, WriteStorage},
    ui::{UiFinder, UiText},
};

#[derive(SystemDesc)]
pub struct Collisions;

impl<'s> System<'s> for Collisions {
    type SystemData = (
        WriteStorage<'s, MovementState>,
        ReadStorage<'s, Ball>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, PlayerType>,
        ReadStorage<'s, Net>,
        ReadStorage<'s, CollisionBox>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, UiText>,
        UiFinder<'s>,
        Write<'s, Score>,
    );

    fn run(
        &mut self,
        (
            mut movement_states,
            balls,
            players,
            player_types,
            nets,
            collision_boxes,
            transforms,
            mut ui_texts,
            ui_finder,
            mut score,
        ): Self::SystemData,
    ) {
        // TODO: some semblance of efficiency for all this.
        // These handling collisions with balls, specifically.
        for (movement_state, _ball, ball_collision, ball_transform) in
            (&mut movement_states, &balls, &collision_boxes, &transforms).join()
        {
            // Handle collisions with players (i.e. kicks)
            for (player, player_type, player_collision, player_transform) in
                (&players, &player_types, &collision_boxes, &transforms).join()
            {
                if collision_box::are_colliding(
                    ball_collision,
                    ball_transform,
                    &player::reduce_collision(
                        player_collision,
                        player_type,
                        movement_state.velocity.norm(),
                    ),
                    player_transform,
                ) {
                    let center_difference =
                        ball_transform.translation().xy() - player_transform.translation().xy();
                    movement_state.velocity =
                        center_difference / center_difference.norm() * player.strength();
                }
            }
            let mut goal_collision = false;
            // Handle collisions with goalines.
            for (net, net_collision, net_transform) in (&nets, &collision_boxes, &transforms).join()
            {
                if collision_box::are_colliding(
                    ball_collision,
                    ball_transform,
                    net_collision,
                    net_transform,
                ) {
                    goal_collision = true;
                    if !score.in_goal_last_frame {
                        let scoreboard = {
                            let scoreboard_entity = ui_finder.find("scoreboard").unwrap();
                            ui_texts.get_mut(scoreboard_entity).unwrap()
                        };
                        match net.side {
                            Side::UpperSide => score.team2 += 1,
                            Side::LowerSide => score.team1 += 1,
                        }
                        scoreboard.text = format!("{} - {}", score.team1, score.team2);
                    }
                }
            }
            score.in_goal_last_frame = goal_collision;
        }
    }
}
