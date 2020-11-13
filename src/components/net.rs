extern crate amethyst;

use amethyst::ecs::{Component, DenseVecStorage};

#[derive(Debug)]
pub enum NetSide {
    Upper,
    Lower,
}

#[derive(Debug)]
pub struct Net {
    side: NetSide,
}

impl Net {
    pub fn new(side: NetSide) -> Net {
        Net { side: side }
    }
}

impl Component for Net {
    type Storage = DenseVecStorage<Self>;
}
