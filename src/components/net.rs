extern crate amethyst;

use amethyst::ecs::{Component, DenseVecStorage};

pub enum NetSide {
    Upper,
    Lower,
}

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
