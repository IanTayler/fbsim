mod fbsim;

use crate::fbsim::FBSim;

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod components;
mod config;
mod systems;

fn main() -> amethyst::Result<()> {
    // Initialization
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    let input_bundle = {
        let binding_config_path = app_root.join("config").join("bindings.ron");
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_config_path)?
    };
    // Setup a basic GameData
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // Plugin for easy rendering of windows.
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.7, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(input_bundle)?
        .with(systems::AnimatePlayer, "animate_player", &[])
        .with(systems::MovePlayers, "move_player", &[]);
    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, FBSim, game_data)?;
    game.run();
    Ok(())
}
