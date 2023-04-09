use rand::prelude::SliceRandom;
use strum_macros::EnumIter;

use crate::graphics::sprites::TILE;

pub enum ElementSize {
    Small,
    Medium,
    Large,
}

impl ElementSize {
    pub fn get_sprite(&self) -> &[TILE] {
        match self {
            ElementSize::Small => [NO_NAME].choose(&mut rand::thread_rng()),
            ElementSize::Medium => [NO_NAME].choose(&mut rand::thread_rng()),
            ElementSize::Large => [NO_NAME].choose(&mut rand::thread_rng()),
        }.unwrap()
    }
}

#[derive(EnumIter)]
pub enum Layouts {
    Layout1,
    Layout2,
}

impl Layouts {
    pub fn get_elements(&self) -> Vec<(ElementSize, usize, usize)> {
        match self {
            Layouts::Layout1 => vec![
                (ElementSize::Small, 1, 3),
                (ElementSize::Medium, 4, 7),
            ],
            Layouts::Layout2 => vec![
                (ElementSize::Large, 2, 8),
            ]
        }
    }
}

const NO_NAME: [TILE; 14] = [
    (0, 1, 455, 0, 3, true, 1),
    (1, 1, 206, 0, 3, true, 2),
    (2, 1, 231, 0, 3, false, 3),
    (3, 1, 302, 0, 3, false, 1),
    (4, 1, 231, 0, 3, true, 3),
    (5, 1, 206, 0, 3, true, 2),
    (6, 1, 455, 0, 3, false, 1),
    (0, 0, 455, 0, 3, false, 3),
    (1, 0, 206, 0, 3, true, 0),
    (2, 0, 231, 0, 3, false, 0),
    (3, 0, 129, 0, 3, false, 2),
    (4, 0, 231, 0, 3, false, 0),
    (5, 0, 271, 0, 3, false, 3),
    (6, 0, 455, 0, 3, true, 3),
];