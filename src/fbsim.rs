use crate::{components, config};
use amethyst::{
    animation::AnimationSetPrefab,
    assets::{AssetStorage, Loader, PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    core::transform::Transform,
    derive::PrefabData,
    ecs::{Entity, Read, ReadExpect},
    error::Error,
    prelude::*,
    renderer::{
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
        Camera,
    },
    ui,
};
use serde::{Deserialize, Serialize};

/// Animation ids used in a AnimationSet
#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    PlayerRun,
    PlayerStand,
}

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub enum FieldSceneExtras {
    PlayerData {
        /// Player info.
        player: components::Player,
        /// Human component. None if not human.
        human: Option<components::Human>,
        /// Robot component. None if not robot.
        robot: Option<components::Robot>,
    },
    BallData {
        /// Ball component.
        ball: components::Ball,
    },
    NetData {
        /// Net component.
        net: components::Net,
    },
    StaticData,
}

/// Data for entities in the Field (Pitch?) scene.
#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct FieldSceneData {
    /// Information for rendering a scene with sprites
    sprite_scene: SpriteScenePrefab,
    /// Аll animations that can be run on the entity
    animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
    /// Information about collision box. None if no collisions.
    collision_box: Option<components::CollisionBox>,
    /// Other information including special components.
    extras: FieldSceneExtras,
    /// To describe how the object is moving.
    movement_state: Option<components::MovementState>,
}

fn initialize_field(world: &mut World, progress_counter: &mut ProgressCounter) {
    let field_prefab = world.exec(|loader: PrefabLoader<'_, FieldSceneData>| {
        loader.load("sprites/field_ichi_1.ron", RonFormat, progress_counter)
    });
    world.create_entity().with(field_prefab).build();
}

fn initialize_score(world: &mut World, progress_counter: &mut ProgressCounter) {
    // If we can't load the font just let it crash.
    let font = world.exec(
        |(loader, asset_storage): (
            ReadExpect<'_, Loader>,
            Read<'_, AssetStorage<ui::FontAsset>>,
        )| {
            loader.load(
                "fonts/slkscr.ttf",
                ui::TtfFormat,
                progress_counter,
                &asset_storage,
            )
        },
    );
    let text = ui::UiText::new(
        font,
        "0 - 0".to_string(),
        [0.0, 0.0, 0.0, 1.0],
        25.0,
        ui::LineMode::Single,
        ui::Anchor::BottomLeft,
    );
    let ui_transform = ui::UiTransform::new(
        String::from("scoreboard"), // id
        ui::Anchor::TopLeft,        // anchor
        ui::Anchor::TopLeft,        // pivot
        20.0,                       // x
        0.0,                        // y
        0.4,                        // z
        100.0,                      // width
        30.0,                       // height
    );
    world.create_entity().with(text).with(ui_transform).build();
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
#[derive(Default)]
pub struct FieldState {
    pub progress_counter: ProgressCounter,
}

impl FieldState {
    pub fn new() -> Self {
        FieldState {
            progress_counter: ProgressCounter::new(),
        }
    }
}

impl SimpleState for FieldState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialize_field(world, &mut self.progress_counter);
        initialize_camera(world);
        initialize_score(world, &mut self.progress_counter);
    }
}
