mod fbsim;

use crate::fbsim::{AnimationId, FieldSceneData, FieldState};

use amethyst::{
    animation::AnimationBundle,
    assets::PrefabLoaderSystemDesc,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle, SpriteRender,
    },
    utils::application_root_dir,
};

mod components;
mod config;
mod rectangle;
mod systems;
mod utils;

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
        .with_system_desc(
            PrefabLoaderSystemDesc::<FieldSceneData>::default(),
            "scene_loader",
            &[],
        )
        .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new(
            "sprite_animation_control",
            "sprite_sampler_interpolation",
        ))?
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
        .with(
            systems::Collisions,
            "collisions",
            &["animate_player", "input_system"],
        )
        .with(systems::MoveObjects, "move_objects", &["collisions"])
        .with(systems::InputMovement, "input_movement", &["input_system"])
        .with(systems::InputActions, "input_actions", &["input_system"]);
    let assets_dir = app_root.join("assets");
    let mut game = Application::new(assets_dir, FieldState::new(), game_data)?;
    game.run();
    Ok(())
}
