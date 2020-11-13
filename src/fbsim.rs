use amethyst::{
    animation::AnimationSetPrefab,
    assets::{PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    core::transform::Transform,
    derive::PrefabData,
    ecs::Entity,
    error::Error,
    prelude::*,
    renderer::{
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
        Camera,
    },
};
use serde::{Deserialize, Serialize};

use crate::components;
use crate::config;

/// Animation ids used in a AnimationSet
#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    PlayerRun,
    PlayerJump,
    PlayerCelebrate,
    PlayerStand,
}

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub enum FieldSceneExtras {
    PlayerData {
        /// Player info.
        player: components::Player,
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
    /// Other information including special components.
    extras: FieldSceneExtras,
}

fn initialize_field(world: &mut World, progress_counter: &mut ProgressCounter) {
    let field_prefab = world.exec(|loader: PrefabLoader<'_, FieldSceneData>| {
        loader.load("sprites/field_ichi_1.ron", RonFormat, progress_counter)
    });
    world.create_entity().with(field_prefab).build();
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
pub struct FBSim {
    pub progress_counter: ProgressCounter,
}

impl FBSim {
    pub fn new() -> FBSim {
        FBSim {
            progress_counter: ProgressCounter::new(),
        }
    }
}

impl SimpleState for FBSim {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<components::Net>();
        initialize_field(world, &mut self.progress_counter);
        initialize_camera(world);
    }
}
