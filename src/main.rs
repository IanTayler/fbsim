mod fbsim;

use crate::fbsim::FBSim;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod systems;

fn main() -> amethyst::Result<()> {
    // Initialization
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;
    let display_config_path = app_root.join("config").join("display.ron");
    // Setup a basic GameData
    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                // Plugin for easy rendering of windows.
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.8, 0.0, 1.0]),
                )
                // RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
                .with_plugin(RenderFlat2D::default()),
        )?
        .with(systems::AnimatePlayer, "animate_player", &[]);
    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, FBSim, game_data)?;
    game.run();
    Ok(())
}
