extern crate amethyst;

use crate::fbsim::Player;
use amethyst::{
    core::timing::Time,
    derive::SystemDesc,
    ecs::{Join, Read, System, SystemData, WriteStorage},
    renderer::SpriteRender,
};

#[derive(SystemDesc)]
pub struct AnimatePlayer;

impl<'s> System<'s> for AnimatePlayer {
    type SystemData = (
        WriteStorage<'s, Player>,
        WriteStorage<'s, SpriteRender>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut players, mut sprite_renders, time): Self::SystemData) {
        for (player, sprite_render) in (&mut players, &mut sprite_renders).join() {
            player.add_delta(time.delta_seconds());
            sprite_render.sprite_number = player.frame();
        }
    }
}
