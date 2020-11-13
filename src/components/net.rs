extern crate amethyst;

use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub struct Net;

impl Component for Net {
    type Storage = DenseVecStorage<Self>;
}
