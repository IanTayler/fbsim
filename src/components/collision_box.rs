use crate::rectangle;
use amethyst::{
    assets::PrefabData,
    core::{math, Transform},
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize, PrefabData)]
#[prefab(Component)]
pub struct CollisionBox {
    /// Distance from the center to the upper left.
    /// Note: probably negative.
    pub upper_left_distance: math::Vector2<f32>,
    /// Distance from the center to the lower right.
    pub lower_right_distance: math::Vector2<f32>,
}

impl CollisionBox {
    pub fn rectangle(&self, transform: &Transform) -> math::Vector4<f32> {
        let translation = transform.translation().xy();
        math::Vector4::new(
            translation.x + self.upper_left_distance.x,
            translation.y + self.upper_left_distance.y,
            translation.x + self.lower_right_distance.x,
            translation.y + self.lower_right_distance.y,
        )
    }
}

impl Component for CollisionBox {
    type Storage = DenseVecStorage<Self>;
}

impl rectangle::Rectangle<f32> for CollisionBox {
    fn upper_left(&self) -> math::Vector2<f32> {
        self.upper_left_distance
    }

    fn lower_right(&self) -> math::Vector2<f32> {
        self.lower_right_distance
    }

    fn center(&self) -> math::Vector2<f32> {
        (self.upper_left_distance + self.lower_right_distance) / 2.0
    }
}

pub fn are_colliding(
    collision1: &CollisionBox,
    transform1: &Transform,
    collision2: &CollisionBox,
    transform2: &Transform,
) -> bool {
    rectangle::overlap(
        collision1.rectangle(transform1),
        collision2.rectangle(transform2),
    )
}
