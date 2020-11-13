extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::components;
use crate::config;

fn load_sprite_sheet(world: &mut World, sprite_name: String) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("sprites/{}.png", sprite_name),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("sprites/{}_desc.ron", sprite_name),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_players(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(config::SCREEN_WIDTH / 2.0, config::SCREEN_HEIGHT / 2.0, 0.0);
    let player_sprite_render = SpriteRender::new(sprite_sheet_handle, 2);
    world
        .create_entity()
        .with(components::Player::new())
        .with(transform)
        .with(player_sprite_render.clone())
        .build();
}

fn initialize_nets(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut lower_transform = Transform::default();
    let mut upper_transform = Transform::default();
    let x_point = config::SCREEN_WIDTH / 2.0;
    lower_transform.set_translation_xyz(x_point, config::NET_HEIGHT / 2.0, 0.1);
    upper_transform.set_translation_xyz(
        x_point,
        config::SCREEN_HEIGHT - config::NET_HEIGHT / 2.0,
        -0.1,
    );
    let lower_net_sprite_render = SpriteRender::new(sprite_sheet_handle.clone(), 1);
    let upper_net_sprite_render = SpriteRender::new(sprite_sheet_handle.clone(), 0);
    world
        .create_entity()
        .with(components::Net::new(components::net::NetSide::Lower))
        .with(lower_transform)
        .with(lower_net_sprite_render.clone())
        .build();
    world
        .create_entity()
        .with(components::Net::new(components::net::NetSide::Upper))
        .with(upper_transform)
        .with(upper_net_sprite_render.clone())
        .build();
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(config::SCREEN_WIDTH / 2.0, config::SCREEN_HEIGHT / 2.0, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(
            config::SCREEN_WIDTH,
            config::SCREEN_HEIGHT,
        ))
        .with(transform)
        .build();
}

// Define the game state.
pub struct FBSim;

impl SimpleState for FBSim {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<components::Net>();
        let player_sprite_sheet_handle = load_sprite_sheet(world, String::from("player_ichi_1"));
        let nets_sprite_sheet_handle = load_sprite_sheet(world, String::from("nets_ichi_1"));
        initialize_players(world, player_sprite_sheet_handle);
        initialize_nets(world, nets_sprite_sheet_handle);
        initialize_camera(world);
    }
}
