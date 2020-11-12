extern crate amethyst;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

pub const PLAYER_WIDTH: f32 = 16.0;
pub const PLAYER_HEIGHT: f32 = 16.0;
pub const SCREEN_WIDTH: f32 = 100.0;
pub const SCREEN_HEIGHT: f32 = 100.0;
// TODO: move this to configuration files.
// Alternatively, use amethysts' native animation types.
const PLAYER_ANIMATION_TIME: f32 = 0.5;
const PLAYER_ANIMATION_FRAMES: usize = 8;
const PLAYER_ANIMATION_FRAME_VECTOR: [usize; PLAYER_ANIMATION_FRAMES] = [2, 3, 0, 3, 2, 4, 1, 4];

pub struct Player {
    pub height: f32,
    pub width: f32,
    moment: f32,
}

impl Player {
    fn new() -> Player {
        Player {
            width: PLAYER_WIDTH,
            height: PLAYER_HEIGHT,
            moment: 0.0,
        }
    }

    pub fn add_delta(&mut self, delta: f32) {
        self.moment = (self.moment + delta / PLAYER_ANIMATION_TIME).fract()
    }

    pub fn frame(&self) -> usize {
        let scaled_moment = self.moment * (PLAYER_ANIMATION_FRAMES + 1) as f32;
        let position = scaled_moment.trunc() as usize;
        // The modulo wouldn't be necessary if not for floating point rounding errors.
        PLAYER_ANIMATION_FRAME_VECTOR[position % PLAYER_ANIMATION_FRAMES]
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

fn load_player_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/player_ichi_1.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/player_ichi_1_desc.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialize_players(world: &mut World, sprite_sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0, 0.0);
    let player_sprite_render = SpriteRender::new(sprite_sheet_handle, 2);
    world
        .create_entity()
        .with(Player::new())
        .with(transform)
        .with(player_sprite_render.clone())
        .build();
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0, 1.0);
    world
        .create_entity()
        .with(Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT))
        .with(transform)
        .build();
}

// Define the game state.
pub struct FBSim;

impl SimpleState for FBSim {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let sprite_sheet_handle = load_player_sprite_sheet(world);
        world.register::<Player>();
        initialize_players(world, sprite_sheet_handle);
        initialize_camera(world);
    }
}
