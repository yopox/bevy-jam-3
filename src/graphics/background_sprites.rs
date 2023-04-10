use rand::prelude::{IteratorRandom, SliceRandom};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::graphics::sprites::TILE;
use crate::util::background::LAYOUT_HEIGHT;

pub enum ElementSize {
    Big,
    FxF,
    Ground,
    Elements,
    Medium,
    TxT,
    Horizontal,
}

impl ElementSize {
    pub fn get_sprite(&self) -> &[TILE] {
        match self {
            ElementSize::Big => [LAVA.as_ref(), CRACK.as_ref(), FOREST.as_ref(), MOUNT.as_ref()]
                .choose(&mut rand::thread_rng()).unwrap(),
            ElementSize::FxF => [ROCK.as_ref(), GRAVE.as_ref(),GRASS2.as_ref(), FLOWER2.as_ref(), ROCKS.as_ref()]
                .choose(&mut rand::thread_rng()).unwrap(),
            ElementSize::Ground => [GROUND.as_ref(), GROUND2.as_ref(), GROUND3.as_ref(), GROUND4.as_ref()]
                .choose(&mut rand::thread_rng()).unwrap(),
            ElementSize::Elements => [CHARIOT.as_ref(), GRAVE.as_ref(), SKULL_SIGN.as_ref(), CHARIOT.as_ref()]
                .choose(&mut rand::thread_rng()).unwrap(),
            ElementSize::Medium => [LAVA_LAKE.as_ref(), LAVA2.as_ref(), MUSH.as_ref()]
                .choose(&mut rand::thread_rng()).unwrap(),
            ElementSize::TxT => [TFLOWER.as_ref(), TROCK.as_ref(), TGRASS.as_ref(), TLAVA.as_ref(), TMUSH.as_ref(), TBUSH.as_ref(), TMARIO.as_ref(), GRASS.as_ref(), FLOWER.as_ref(), BROKEN_RAILS.as_ref()]
                .choose(&mut rand::thread_rng()).unwrap(),
            ElementSize::Horizontal => [RUINS.as_ref(), HCHARIOT.as_ref(), YOPOX.as_ref(), HADRI.as_ref(), VICO.as_ref(), SIGNCRACK.as_ref(), ROCKS2.as_ref()]
                .choose(&mut rand::thread_rng()).unwrap(),
        }
    }
}

#[derive(EnumIter)]
pub enum Layouts {
    Layout1,
    Layout2,
    Layout2Clone,
    Layout3,
    Layout4,
    Layout5,
    Layout6,
}

impl Layouts {
    pub fn random() -> Self {
        Self::iter().choose(&mut rand::thread_rng()).unwrap()
        //Layouts::Layout4
    }

    pub fn get_elements(&self) -> Vec<(ElementSize, usize, usize)> {
        match self {
            Layouts::Layout1 => vec![
                (ElementSize::Big, 1, 0),
                (ElementSize::FxF, 7, 9),
                (ElementSize::Ground, 6, 3),
                (ElementSize::TxT, 9, 1),
            ],

            Layouts::Layout2 | Layouts::Layout2Clone => vec![
                (ElementSize::FxF, 2, 9),
                (ElementSize::Ground, 3, 5),
                (ElementSize::Ground, 5, 7),
                (ElementSize::Ground, 1, 7),
                (ElementSize::Elements, 1, 1),
                (ElementSize::TxT, 7, 0),
                (ElementSize::TxT, 9, 4),
            ],

            Layouts::Layout3 => vec![
                (ElementSize::Big, 1, 0),
                (ElementSize::FxF, 7, 10),
                (ElementSize::Ground, 7, 3),
                (ElementSize::TxT, 7, 7),
            ],

            Layouts::Layout4 => vec![
                (ElementSize::Medium, 4, 8),
                (ElementSize::FxF, 1, 5),
                (ElementSize::Ground, 6, 5),
                (ElementSize::TxT, 8, 3),
                (ElementSize::TxT, 0, 0),
            ],

            Layouts::Layout5 => vec![
                (ElementSize::Horizontal, 3, 8),
                (ElementSize::FxF, 0, 2),
                (ElementSize::Ground, 7, 6),
                (ElementSize::TxT, 9, 2),
                (ElementSize::TxT, 4, 0),
            ],

            Layouts::Layout6 => vec![
                (ElementSize::FxF, 0, 2),
                (ElementSize::FxF, 7, 6),
                (ElementSize::FxF, 9, 2),
                (ElementSize::FxF, 4, 0),
            ],

            // Layouts::Layout2 => vec![
            //     (ElementSize::Cloud, 2, 8),
            //     (ElementSize::Cloud, 2, 8),
            //]
        }
    }
}

// 10x6
const SIGNCRACK: [TILE ; 60] =[
    (0, 5, 0, 0, 4, true, 0),
    (1, 5, 0, 0, 4, true, 2),
    (2, 5, 0, 0, 4, true, 3),
    (3, 5, 0, 0, 4, true, 0),
    (4, 5, 293, 0, 3, false, 1),
    (5, 5, 163, 0, 3, true, 0),
    (6, 5, 463, 0, 3, true, 0),
    (7, 5, 0, 0, 1, true, 1),
    (8, 5, 0, 0, 1, true, 1),
    (9, 5, 0, 0, 1, false, 0),
    (0, 4, 0, 0, 4, true, 0),
    (1, 4, 293, 0, 2, false, 0),
    (2, 4, 0, 0, 4, true, 3),
    (3, 4, 698, 0, 3, false, 1),
    (4, 4, 711, 0, 3, false, 0),
    (5, 4, 168, 4, 3, true, 3),
    (6, 4, 778, 3, 4, true, 0),
    (7, 4, 169, 4, 3, true, 1),
    (8, 4, 234, 0, 3, true, 2),
    (9, 4, 0, 0, 1, false, 0),
    (0, 3, 0, 0, 4, false, 1),
    (1, 3, 0, 0, 4, false, 1),
    (2, 3, 834, 0, 3, false, 1),
    (3, 3, 708, 3, 4, true, 3),
    (4, 3, 168, 4, 3, false, 2),
    (5, 3, 773, 3, 4, true, 2),
    (6, 3, 777, 4, 1, true, 0),
    (7, 3, 815, 4, 1, true, 3),
    (8, 3, 163, 3, 4, true, 3),
    (9, 3, 239, 0, 3, true, 3),
    (0, 2, 530, 0, 4, true, 1),
    (1, 2, 14, 0, 3, true, 0),
    (2, 2, 530, 0, 4, true, 0),
    (3, 2, 773, 0, 3, true, 2),
    (4, 2, 806, 3, 4, false, 3),
    (5, 2, 774, 3, 4, true, 1),
    (6, 2, 835, 3, 4, true, 0),
    (7, 2, 778, 3, 4, true, 2),
    (8, 2, 234, 0, 3, true, 1),
    (9, 2, 230, 0, 3, true, 1),
    (0, 1, 530, 0, 4, true, 2),
    (1, 1, 244, 0, 3, true, 0),
    (2, 1, 530, 0, 4, true, 3),
    (3, 1, 323, 0, 3, true, 2),
    (4, 1, 494, 0, 3, true, 0),
    (5, 1, 778, 0, 3, true, 2),
    (6, 1, 715, 0, 3, true, 2),
    (7, 1, 173, 0, 3, true, 2),
    (8, 1, 0, 0, 1, true, 1),
    (9, 1, 0, 0, 4, false, 0),
    (0, 0, 0, 0, 12, false, 0),
    (1, 0, 244, 0, 4, true, 0),
    (2, 0, 0, 0, 4, true, 0),
    (3, 0, 397, 0, 3, true, 3),
    (4, 0, 0, 0, 4, true, 0),
    (5, 0, 0, 0, 12, false, 0),
    (6, 0, 0, 0, 12, false, 3),
    (7, 0, 0, 0, 1, true, 1),
    (8, 0, 878, 0, 2, false, 0),
    (9, 0, 0, 0, 1, false, 0),
];


// 9x6
const HADRI: [TILE ; 54] = [
    (0, 5, 462, 0, 12, true, 0),
    (1, 5, 0, 0, 3, true, 0),
    (2, 5, 0, 0, 11, true, 0),
    (3, 5, 0, 0, 11, true, 0),
    (4, 5, 0, 0, 3, true, 0),
    (5, 5, 0, 0, 11, false, 1),
    (6, 5, 643, 0, 11, true, 0),
    (7, 5, 0, 0, 1, false, 0),
    (8, 5, 0, 0, 1, false, 0),
    (0, 4, 0, 0, 4, true, 1),
    (1, 4, 0, 0, 4, true, 0),
    (2, 4, 0, 0, 4, true, 0),
    (3, 4, 783, 0, 11, true, 0),
    (4, 4, 0, 0, 12, true, 0),
    (5, 4, 0, 0, 11, false, 0),
    (6, 4, 0, 0, 11, true, 0),
    (7, 4, 0, 0, 11, false, 0),
    (8, 4, 0, 0, 1, false, 0),
    (0, 3, 0, 0, 4, true, 1),
    (1, 3, 0, 0, 4, false, 2),
    (2, 3, 365, 0, 4, false, 3),
    (3, 3, 363, 0, 4, false, 2),
    (4, 3, 363, 0, 4, false, 2),
    (5, 3, 362, 0, 4, false, 1),
    (6, 3, 173, 0, 4, false, 0),
    (7, 3, 724, 0, 4, false, 0),
    (8, 3, 0, 0, 4, false, 1),
    (0, 2, 0, 0, 4, true, 0),
    (1, 2, 365, 0, 4, true, 0),
    (2, 2, 363, 0, 4, true, 0),
    (3, 2, 904, 4, 3, true, 0),
    (4, 2, 897, 4, 3, true, 2),
    (5, 2, 900, 4, 3, true, 0),
    (6, 2, 914, 4, 3, true, 0),
    (7, 2, 905, 4, 3, true, 0),
    (8, 2, 365, 0, 4, false, 0),
    (0, 1, 365, 0, 4, true, 0),
    (1, 1, 363, 0, 4, true, 0),
    (2, 1, 363, 0, 4, true, 0),
    (3, 1, 363, 0, 4, true, 0),
    (4, 1, 363, 0, 4, true, 0),
    (5, 1, 363, 0, 4, true, 0),
    (6, 1, 364, 0, 4, false, 0),
    (7, 1, 363, 0, 4, true, 0),
    (8, 1, 363, 0, 4, false, 0),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 0, 0, 12, true, 2),
    (2, 0, 0, 0, 3, true, 0),
    (3, 0, 0, 0, 3, true, 0),
    (4, 0, 0, 0, 3, true, 0),
    (5, 0, 0, 0, 3, true, 0),
    (6, 0, 0, 0, 3, true, 0),
    (7, 0, 427, 0, 12, true, 2),
    (8, 0, 0, 0, 1, false, 0),
];

// 9x6
const YOPOX: [TILE ; 54] = [
    (0, 5, 0, 0, 1, false, 0),
    (1, 5, 0, 0, 3, true, 0),
    (2, 5, 0, 0, 3, true, 0),
    (3, 5, 0, 0, 3, true, 0),
    (4, 5, 0, 0, 3, true, 0),
    (5, 5, 0, 0, 3, true, 0),
    (6, 5, 462, 0, 12, true, 0),
    (7, 5, 0, 0, 1, false, 0),
    (8, 5, 0, 0, 1, false, 0),
    (0, 4, 0, 0, 4, true, 1),
    (1, 4, 783, 0, 7, true, 0),
    (2, 4, 0, 0, 4, true, 2),
    (3, 4, 0, 0, 4, true, 2),
    (4, 4, 0, 0, 4, true, 2),
    (5, 4, 0, 0, 4, true, 2),
    (6, 4, 0, 0, 4, false, 2),
    (7, 4, 0, 0, 4, false, 0),
    (8, 4, 0, 0, 1, false, 0),
    (0, 3, 362, 0, 4, true, 1),
    (1, 3, 363, 0, 4, false, 2),
    (2, 3, 363, 0, 4, false, 2),
    (3, 3, 363, 0, 4, false, 2),
    (4, 3, 363, 0, 4, false, 2),
    (5, 3, 362, 0, 4, false, 1),
    (6, 3, 0, 0, 4, false, 1),
    (7, 3, 643, 0, 7, true, 0),
    (8, 3, 0, 0, 1, false, 0),
    (0, 2, 363, 0, 4, true, 0),
    (1, 2, 921, 4, 3, true, 1),
    (2, 2, 911, 4, 3, false, 2),
    (3, 2, 912, 4, 3, false, 0),
    (4, 2, 911, 4, 3, true, 0),
    (5, 2, 920, 4, 3, true, 0),
    (6, 2, 365, 0, 4, false, 0),
    (7, 2, 0, 0, 4, false, 0),
    (8, 2, 0, 0, 1, false, 0),
    (0, 1, 363, 0, 4, true, 0),
    (1, 1, 363, 0, 4, true, 0),
    (2, 1, 363, 0, 4, true, 0),
    (3, 1, 363, 0, 4, true, 0),
    (4, 1, 363, 0, 4, true, 0),
    (5, 1, 363, 0, 4, true, 0),
    (6, 1, 364, 0, 4, false, 0),
    (7, 1, 363, 0, 4, true, 0),
    (8, 1, 365, 0, 4, false, 0),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 427, 0, 12, true, 2),
    (2, 0, 0, 0, 3, true, 0),
    (3, 0, 0, 0, 3, true, 0),
    (4, 0, 0, 0, 3, true, 0),
    (5, 0, 0, 0, 3, true, 0),
    (6, 0, 0, 0, 3, true, 0),
    (7, 0, 0, 0, 1, false, 0),
    (8, 0, 0, 0, 1, false, 0),
];

// 9x6
const VICO: [TILE ; 54] = [
    (0, 5, 0, 0, 12, true, 0),
    (1, 5, 0, 0, 3, true, 0),
    (2, 5, 643, 0, 15, true, 0),
    (3, 5, 0, 0, 11, true, 0),
    (4, 5, 0, 0, 3, true, 0),
    (5, 5, 0, 0, 11, false, 1),
    (6, 5, 0, 0, 15, true, 0),
    (7, 5, 0, 0, 1, false, 0),
    (8, 5, 462, 0, 12, true, 0),
    (0, 4, 0, 0, 4, true, 1),
    (1, 4, 0, 0, 4, true, 0),
    (2, 4, 0, 0, 4, true, 0),
    (3, 4, 0, 0, 15, true, 0),
    (4, 4, 783, 0, 15, true, 0),
    (5, 4, 724, 0, 4, true, 0),
    (6, 4, 0, 0, 11, true, 0),
    (7, 4, 0, 0, 11, false, 0),
    (8, 4, 0, 0, 1, false, 0),
    (0, 3, 0, 0, 4, true, 1),
    (1, 3, 0, 0, 4, false, 2),
    (2, 3, 0, 0, 4, false, 3),
    (3, 3, 365, 0, 4, false, 3),
    (4, 3, 363, 0, 4, false, 2),
    (5, 3, 364, 0, 4, false, 2),
    (6, 3, 365, 0, 4, true, 3),
    (7, 3, 0, 0, 4, false, 0),
    (8, 3, 0, 0, 4, false, 1),
    (0, 2, 0, 0, 4, true, 0),
    (1, 2, 365, 0, 4, true, 0),
    (2, 2, 363, 0, 4, true, 0),
    (3, 2, 918, 4, 3, true, 3),
    (4, 2, 905, 4, 3, true, 0),
    (5, 2, 899, 4, 3, true, 0),
    (6, 2, 911, 4, 3, true, 1),
    (7, 2, 363, 0, 4, true, 0),
    (8, 2, 724, 0, 4, true, 0),
    (0, 1, 0, 0, 4, true, 1),
    (1, 1, 363, 0, 4, true, 0),
    (2, 1, 363, 0, 4, true, 0),
    (3, 1, 363, 0, 4, true, 0),
    (4, 1, 363, 0, 4, true, 0),
    (5, 1, 363, 0, 4, true, 0),
    (6, 1, 364, 0, 4, false, 0),
    (7, 1, 363, 0, 4, true, 0),
    (8, 1, 365, 0, 4, false, 0),
    (0, 0, 427, 0, 12, false, 2),
    (1, 0, 0, 0, 12, true, 2),
    (2, 0, 0, 0, 3, true, 0),
    (3, 0, 0, 0, 3, true, 0),
    (4, 0, 0, 0, 3, true, 0),
    (5, 0, 0, 0, 3, true, 0),
    (6, 0, 0, 0, 3, true, 0),
    (7, 0, 0, 0, 12, true, 2),
    (8, 0, 0, 0, 1, false, 0),
];

// 10x6
const HCHARIOT: [TILE ; 60] = [
    (0, 5, 724, 0, 4, true, 0),
    (1, 5, 0, 0, 12, true, 2),
    (2, 5, 293, 0, 4, true, 3),
    (3, 5, 401, 0, 1, true, 0),
    (4, 5, 397, 0, 1, false, 3),
    (5, 5, 293, 0, 1, true, 2),
    (6, 5, 0, 0, 1, false, 3),
    (7, 5, 0, 0, 1, true, 1),
    (8, 5, 0, 0, 1, true, 1),
    (9, 5, 0, 0, 1, false, 0),
    (0, 4, 0, 0, 12, true, 0),
    (1, 4, 0, 0, 12, true, 2),
    (2, 4, 400, 0, 4, true, 3),
    (3, 4, 397, 0, 4, true, 1),
    (4, 4, 397, 0, 4, false, 3),
    (5, 4, 400, 0, 4, true, 3),
    (6, 4, 0, 0, 12, true, 2),
    (7, 4, 0, 0, 1, true, 1),
    (8, 4, 0, 0, 12, true, 0),
    (9, 4, 0, 0, 1, false, 0),
    (0, 3, 299, 0, 3, false, 1),
    (1, 3, 331, 0, 3, false, 1),
    (2, 3, 53, 4, 1, true, 1),
    (3, 3, 53, 4, 1, true, 3),
    (4, 3, 331, 0, 3, false, 1),
    (5, 3, 55, 4, 1, true, 3),
    (6, 3, 331, 0, 3, false, 1),
    (7, 3, 331, 0, 3, false, 1),
    (8, 3, 299, 0, 3, false, 3),
    (9, 3, 0, 0, 1, false, 0),
    (0, 2, 0, 0, 1, true, 1),
    (1, 2, 0, 0, 12, true, 2),
    (2, 2, 397, 0, 4, false, 1),
    (3, 2, 397, 0, 4, false, 2),
    (4, 2, 401, 0, 4, true, 0),
    (5, 2, 400, 0, 4, true, 0),
    (6, 2, 0, 0, 12, true, 2),
    (7, 2, 0, 0, 12, false, 3),
    (8, 2, 0, 0, 1, true, 1),
    (9, 2, 456, 0, 4, true, 3),
    (0, 1, 0, 0, 1, true, 1),
    (1, 1, 0, 0, 1, true, 3),
    (2, 1, 397, 0, 1, false, 2),
    (3, 1, 400, 0, 4, true, 1),
    (4, 1, 293, 0, 4, true, 1),
    (5, 1, 0, 0, 12, false, 0),
    (6, 1, 0, 0, 12, true, 2),
    (7, 1, 0, 0, 1, true, 1),
    (8, 1, 0, 0, 1, true, 1),
    (9, 1, 0, 0, 1, false, 0),
    (0, 0, 0, 0, 12, false, 0),
    (1, 0, 0, 0, 1, true, 1),
    (2, 0, 401, 0, 1, true, 0),
    (3, 0, 293, 0, 1, true, 0),
    (4, 0, 0, 0, 12, true, 0),
    (5, 0, 0, 0, 12, false, 0),
    (6, 0, 0, 0, 12, false, 3),
    (7, 0, 0, 0, 1, true, 1),
    (8, 0, 724, 0, 4, true, 0),
    (9, 0, 0, 0, 1, false, 0),
];

//6x11
const MOUNT: [TILE ; 66] = [
    (0, 10, 397, 0, 3, false, 0),
    (1, 10, 239, 0, 3, true, 0),
    (2, 10, 173, 0, 4, true, 0),
    (3, 10, 0, 0, 3, true, 1),
    (4, 10, 0, 0, 3, true, 0),
    (5, 10, 0, 0, 3, true, 0),
    (0, 9, 239, 0, 3, true, 1),
    (1, 9, 239, 3, 4, true, 0),
    (2, 9, 168, 3, 4, false, 3),
    (3, 9, 10, 0, 4, true, 3),
    (4, 9, 11, 0, 3, true, 1),
    (5, 9, 0, 0, 4, true, 1),
    (0, 8, 130, 0, 4, true, 3),
    (1, 8, 168, 3, 4, true, 0),
    (2, 8, 397, 4, 3, false, 0),
    (3, 8, 168, 3, 4, true, 2),
    (4, 8, 715, 0, 4, true, 3),
    (5, 8, 459, 0, 12, false, 0),
    (0, 7, 0, 0, 4, true, 1),
    (1, 7, 224, 0, 4, false, 1),
    (2, 7, 397, 4, 3, false, 3),
    (3, 7, 397, 4, 3, false, 2),
    (4, 7, 711, 0, 4, false, 1),
    (5, 7, 0, 0, 4, true, 1),
    (0, 6, 458, 0, 12, false, 0),
    (1, 6, 174, 0, 4, false, 2),
    (2, 6, 168, 3, 4, false, 1),
    (3, 6, 397, 4, 3, false, 2),
    (4, 6, 168, 3, 4, true, 2),
    (5, 6, 0, 0, 4, true, 1),
    (0, 5, 0, 0, 4, true, 1),
    (1, 5, 238, 0, 4, false, 1),
    (2, 5, 98, 4, 3, true, 1),
    (3, 5, 397, 4, 3, false, 2),
    (4, 5, 397, 4, 3, false, 1),
    (5, 5, 724, 0, 4, true, 3),
    (0, 4, 0, 0, 4, true, 1),
    (1, 4, 130, 0, 4, true, 3),
    (2, 4, 168, 3, 4, false, 2),
    (3, 4, 397, 4, 3, false, 0),
    (4, 4, 397, 4, 3, false, 2),
    (5, 4, 70, 0, 4, true, 1),
    (0, 3, 0, 0, 4, true, 1),
    (1, 3, 715, 0, 4, true, 1),
    (2, 3, 397, 4, 3, false, 0),
    (3, 3, 397, 4, 3, false, 3),
    (4, 3, 714, 0, 4, false, 1),
    (5, 3, 0, 0, 4, true, 2),
    (0, 2, 130, 0, 4, true, 2),
    (1, 2, 130, 0, 4, true, 2),
    (2, 2, 397, 4, 3, false, 0),
    (3, 2, 168, 3, 4, false, 0),
    (4, 2, 70, 0, 4, true, 1),
    (5, 2, 0, 0, 4, true, 1),
    (0, 1, 239, 0, 3, true, 1),
    (1, 1, 239, 3, 4, true, 2),
    (2, 1, 397, 4, 3, false, 2),
    (3, 1, 130, 0, 4, true, 1),
    (4, 1, 0, 0, 3, true, 2),
    (5, 1, 11, 0, 3, true, 1),
    (0, 0, 397, 0, 3, false, 1),
    (1, 0, 239, 0, 3, true, 2),
    (2, 0, 459, 0, 12, false, 0),
    (3, 0, 0, 0, 4, true, 1),
    (4, 0, 0, 0, 4, true, 1),
    (5, 0, 0, 0, 3, true, 0),
];

// 6x11
const FOREST: [TILE ; 66] = [
    (0, 10, 397, 0, 3, true, 0),
    (1, 10, 0, 0, 4, true, 0),
    (2, 10, 8, 0, 12, false, 0),
    (3, 10, 9, 0, 12, true, 3),
    (4, 10, 9, 0, 12, false, 0),
    (5, 10, 131, 0, 12, false, 2),
    (0, 9, 0, 0, 4, true, 1),
    (1, 9, 8, 0, 12, true, 1),
    (2, 9, 878, 12, 13, true, 2),
    (3, 9, 98, 12, 13, true, 2),
    (4, 9, 163, 12, 13, true, 3),
    (5, 9, 65, 0, 12, true, 2),
    (0, 8, 0, 0, 13, true, 3),
    (1, 8, 9, 0, 12, true, 1),
    (2, 8, 43, 12, 13, true, 3),
    (3, 8, 871, 12, 13, true, 2),
    (4, 8, 162, 13, 12, true, 1),
    (5, 8, 329, 0, 12, false, 3),
    (0, 7, 10, 0, 12, true, 0),
    (1, 7, 42, 0, 12, true, 3),
    (2, 7, 838, 12, 13, true, 3),
    (3, 7, 41, 13, 12, false, 3),
    (4, 7, 163, 12, 13, false, 3),
    (5, 7, 360, 0, 12, true, 1),
    (0, 6, 42, 0, 12, true, 0),
    (1, 6, 39, 12, 13, true, 0),
    (2, 6, 878, 12, 13, true, 2),
    (3, 6, 106, 12, 13, true, 3),
    (4, 6, 8, 0, 12, true, 3),
    (5, 6, 724, 0, 3, true, 0),
    (0, 5, 40, 0, 12, true, 2),
    (1, 5, 163, 12, 13, false, 2),
    (2, 5, 41, 13, 12, false, 0),
    (3, 5, 98, 12, 13, true, 0),
    (4, 5, 8, 0, 12, true, 0),
    (5, 5, 0, 0, 4, true, 3),
    (0, 4, 131, 0, 12, false, 1),
    (1, 4, 162, 13, 12, true, 1),
    (2, 4, 163, 12, 13, false, 3),
    (3, 4, 9, 13, 12, true, 1),
    (4, 4, 13, 12, 13, true, 3),
    (5, 4, 98, 0, 12, true, 2),
    (0, 3, 0, 0, 4, true, 1),
    (1, 3, 329, 0, 12, false, 0),
    (2, 3, 9, 0, 12, false, 2),
    (3, 3, 9, 0, 12, true, 1),
    (4, 3, 162, 13, 12, true, 1),
    (5, 3, 9, 0, 12, true, 3),
    (0, 2, 0, 0, 4, true, 2),
    (1, 2, 0, 0, 4, true, 2),
    (2, 2, 65, 0, 4, true, 0),
    (3, 2, 130, 0, 12, true, 3),
    (4, 2, 41, 13, 12, false, 0),
    (5, 2, 37, 0, 12, true, 2),
    (0, 1, 330, 0, 3, true, 0),
    (1, 1, 0, 0, 4, true, 2),
    (2, 1, 0, 0, 4, false, 2),
    (3, 1, 37, 0, 12, true, 1),
    (4, 1, 9, 0, 12, true, 1),
    (5, 1, 74, 0, 12, true, 1),
    (0, 0, 0, 0, 13, false, 1),
    (1, 0, 0, 0, 13, true, 2),
    (2, 0, 459, 0, 12, false, 0),
    (3, 0, 0, 0, 4, true, 1),
    (4, 0, 65, 0, 4, true, 2),
    (5, 0, 0, 0, 3, true, 0),
];

//3x3
const TGRASS: [TILE ; 9] =[
    (0, 2, 458, 0, 12, true, 0),
    (1, 2, 0, 0, 4, false, 0),
    (2, 2, 0, 0, 3, false, 0),
    (0, 1, 0, 0, 4, false, 0),
    (1, 1, 0, 0, 3, false, 3),
    (2, 1, 724, 0, 4, true, 0),
    (0, 0, 0, 0, 4, false, 0),
    (1, 0, 462, 0, 12, false, 0),
    (2, 0, 0, 0, 4, false, 0),
];

//3x3
const TMARIO: [TILE ; 9] =[
    (0, 2, 0, 0, 13, false, 3),
    (1, 2, 0, 0, 4, false, 0),
    (2, 2, 396, 0, 17, false, 0),
    (0, 1, 397, 0, 3, true, 0),
    (1, 1, 0, 0, 13, false, 0),
    (2, 1, 428, 0, 13, false, 2),
    (0, 0, 0, 0, 3, true, 0),
    (1, 0, 0, 0, 12, false, 0),
    (2, 0, 425, 0, 12, false, 3),
];

//3x3
const TMUSH: [TILE ; 9] =[
    (0, 2, 0, 0, 4, false, 0),
    (1, 2, 724, 0, 3, false, 0),
    (2, 2, 0, 0, 3, false, 0),
    (0, 1, 844, 0, 4, false, 0),
    (1, 1, 0, 0, 3, false, 3),
    (2, 1, 0, 0, 4, false, 3),
    (0, 0, 0, 0, 4, false, 0),
    (1, 0, 0, 0, 3, false, 3),
    (2, 0, 458, 0, 3, false, 0),
];

//3x3
const TBUSH: [TILE ; 9] =[
    (0, 2, 0, 0, 4, false, 0),
    (1, 2, 0, 0, 4, true, 0),
    (2, 2, 724, 0, 3, true, 0),
    (0, 1, 10, 0, 12, true, 0),
    (1, 1, 9, 0, 12, true, 0),
    (2, 1, 8, 0, 12, true, 0),
    (0, 0, 397, 0, 4, true, 0),
    (1, 0, 0, 0, 4, true, 0),
    (2, 0, 0, 0, 4, true, 3),
];

//3x3
const TLAVA: [TILE ; 12] =[
    (0, 2, 0, 0, 1, false, 0),
    (1, 2, 8, 0, 4, false, 0),
    (2, 2, 463, 0, 4, true, 0),
    (3, 2, 397, 0, 2, false, 3),
    (0, 1, 72, 0, 4, false, 0),
    (1, 1, 73, 4, 2, false, 1),
    (2, 1, 398, 4, 2, true, 0),
    (3, 1, 161, 0, 4, false, 1),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 8, 0, 4, true, 2),
    (2, 0, 238, 0, 4, true, 0),
    (3, 0, 205, 0, 4, true, 3),
];


//3x3
const TFLOWER: [TILE ; 9] = [
    (0, 2, 458, 0, 12, true, 0),
    (1, 2, 0, 0, 4, false, 0),
    (2, 2, 396, 0, 17, false, 0),
    (0, 1, 0, 0, 4, false, 0),
    (1, 1, 0, 0, 3, false, 3),
    (2, 1, 463, 0, 12, false, 0),
    (0, 0, 397, 0, 3, true, 0),
    (1, 0, 0, 0, 12, false, 0),
    (2, 0, 0, 0, 4, false, 0),
];

//3x3
const TROCK: [TILE ; 9] = [
    (0, 2, 293, 0, 3, false, 0),
    (1, 2, 0, 0, 3, false, 3),
    (2, 2, 0, 0, 3, false, 0),
    (0, 1, 0, 0, 3, false, 3),
    (1, 1, 0, 0, 3, false, 3),
    (2, 1, 456, 0, 3, false, 3),
    (0, 0, 330, 0, 4, false, 0),
    (1, 0, 0, 0, 3, false, 3),
    (2, 0, 0, 0, 3, false, 3),
];

// 7x2
const ROCKS2: [TILE; 14] = [
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

// 4x5
const GRAVE: [TILE; 20] = [
    (0, 4, 0, 0, 3, true, 2),
    (1, 4, 173, 0, 3, false, 0),
    (2, 4, 0, 0, 12, true, 0),
    (3, 4, 0, 0, 1, false, 0),
    (0, 3, 87, 0, 3, false, 3),
    (1, 3, 116, 3, 4, false, 2),
    (2, 3, 87, 0, 3, false, 1),
    (3, 3, 0, 0, 1, false, 0),
    (0, 2, 87, 0, 3, false, 3),
    (1, 2, 46, 3, 4, false, 0),
    (2, 2, 87, 0, 3, false, 1),
    (3, 2, 355, 0, 4, true, 0),
    (0, 1, 87, 0, 3, false, 2),
    (1, 1, 87, 0, 3, false, 2),
    (2, 1, 87, 0, 3, false, 2),
    (3, 1, 0, 0, 1, false, 0),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 0, 0, 1, false, 0),
    (2, 0, 67, 0, 12, true, 0),
    (3, 0, 463, 0, 12, true, 0),
];

// 5x12
const CRACK: [TILE; 60] = [
    (0, 11, 838, 0, 3, true, 0),
    (1, 11, 0, 0, 4, true, 3),
    (2, 11, 234, 0, 4, true, 3),
    (3, 11, 834, 0, 4, true, 2),
    (4, 11, 141, 0, 3, false, 0),
    (0, 10, 0, 0, 4, false, 2),
    (1, 10, 231, 0, 3, true, 0),
    (2, 10, 772, 3, 4, true, 0),
    (3, 10, 395, 1, 4, true, 3),
    (4, 10, 234, 0, 4, false, 3),
    (0, 9, 0, 0, 4, false, 2),
    (1, 9, 495, 0, 4, false, 2),
    (2, 9, 239, 4, 1, false, 3),
    (3, 9, 239, 4, 1, false, 1),
    (4, 9, 360, 0, 4, false, 3),
    (0, 8, 0, 0, 4, false, 2),
    (1, 8, 778, 0, 4, false, 3),
    (2, 8, 1023, 1, 1, false, 3),
    (3, 8, 207, 1, 4, false, 1),
    (4, 8, 258, 0, 4, false, 3),
    (0, 7, 293, 0, 3, false, 3),
    (1, 7, 712, 0, 4, false, 3),
    (2, 7, 207, 1, 4, false, 3),
    (3, 7, 230, 1, 4, false, 1),
    (4, 7, 170, 0, 4, false, 1),
    (0, 6, 0, 0, 4, true, 3),
    (1, 6, 710, 0, 4, false, 3),
    (2, 6, 324, 4, 1, false, 2),
    (3, 6, 239, 4, 1, false, 1),
    (4, 6, 234, 0, 4, true, 1),
    (0, 5, 141, 0, 3, false, 2),
    (1, 5, 710, 0, 4, true, 1),
    (2, 5, 168, 4, 1, false, 0),
    (3, 5, 834, 4, 1, false, 3),
    (4, 5, 715, 0, 4, true, 3),
    (0, 4, 234, 0, 4, true, 3),
    (1, 4, 200, 1, 4, false, 3),
    (2, 4, 1023, 1, 1, false, 3),
    (3, 4, 163, 1, 4, true, 1),
    (4, 4, 710, 0, 4, true, 3),
    (0, 3, 170, 0, 4, false, 3),
    (1, 3, 239, 4, 1, false, 3),
    (2, 3, 1023, 1, 1, false, 3),
    (3, 3, 207, 4, 1, false, 3),
    (4, 3, 724, 0, 4, true, 3),
    (0, 2, 258, 0, 4, false, 1),
    (1, 2, 203, 1, 4, false, 3),
    (2, 2, 237, 4, 1, false, 3),
    (3, 2, 264, 0, 4, false, 2),
    (4, 2, 293, 0, 3, false, 3),
    (0, 1, 360, 0, 4, false, 1),
    (1, 1, 202, 1, 4, false, 3),
    (2, 1, 200, 1, 4, false, 0),
    (3, 1, 141, 0, 3, false, 3),
    (4, 1, 0, 0, 4, false, 2),
    (0, 0, 0, 0, 4, false, 2),
    (1, 0, 239, 0, 4, false, 2),
    (2, 0, 234, 0, 4, true, 1),
    (3, 0, 838, 0, 3, true, 3),
    (4, 0, 628, 0, 3, true, 2),
];

// 9x6
const RUINS: [TILE ; 54] = [
    (0, 5, 0, 0, 1, false, 0),
    (1, 5, 0, 0, 1, false, 0),
    (2, 5, 0, 0, 1, false, 0),
    (3, 5, 173, 0, 3, true, 0),
    (4, 5, 365, 0, 3, false, 3),
    (5, 5, 361, 0, 3, true, 2),
    (6, 5, 365, 0, 3, true, 3),
    (7, 5, 0, 0, 1, false, 0),
    (8, 5, 0, 0, 1, false, 0),
    (0, 4, 0, 0, 1, false, 0),
    (1, 4, 0, 0, 1, false, 0),
    (2, 4, 362, 0, 3, true, 1),
    (3, 4, 363, 0, 3, false, 2),
    (4, 4, 363, 0, 3, false, 2),
    (5, 4, 363, 0, 3, false, 2),
    (6, 4, 364, 0, 3, false, 2),
    (7, 4, 362, 0, 3, false, 1),
    (8, 4, 173, 0, 3, true, 0),
    (0, 3, 462, 0, 12, true, 0),
    (1, 3, 0, 0, 12, false, 0),
    (2, 3, 0, 0, 12, false, 0),
    (3, 3, 0, 0, 1, false, 0),
    (4, 3, 458, 0, 12, true, 0),
    (5, 3, 0, 0, 12, false, 0),
    (6, 3, 0, 0, 1, false, 0),
    (7, 3, 0, 0, 1, false, 0),
    (8, 3, 0, 0, 1, false, 0),
    (0, 2, 0, 0, 1, false, 0),
    (1, 2, 459, 0, 12, true, 0),
    (2, 2, 173, 0, 3, true, 0),
    (3, 2, 0, 0, 12, false, 0),
    (4, 2, 459, 0, 12, true, 0),
    (5, 2, 0, 0, 1, false, 0),
    (6, 2, 0, 0, 1, false, 0),
    (7, 2, 462, 0, 12, true, 0),
    (8, 2, 0, 0, 1, false, 0),
    (0, 1, 0, 0, 1, false, 0),
    (1, 1, 362, 0, 3, false, 0),
    (2, 1, 363, 0, 3, false, 0),
    (3, 1, 364, 0, 3, false, 0),
    (4, 1, 364, 0, 3, false, 0),
    (5, 1, 365, 0, 3, false, 0),
    (6, 1, 0, 0, 1, false, 0),
    (7, 1, 0, 0, 1, false, 0),
    (8, 1, 0, 0, 1, false, 0),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 0, 0, 12, false, 0),
    (2, 0, 0, 0, 12, false, 2),
    (3, 0, 0, 0, 12, false, 0),
    (4, 0, 0, 0, 1, false, 0),
    (5, 0, 0, 0, 1, false, 0),
    (6, 0, 0, 0, 1, false, 0),
    (7, 0, 0, 0, 1, false, 0),
    (8, 0, 0, 0, 1, false, 0),
];

// 4x5
const SKULL_SIGN: [TILE ; 20] = [
    (0, 4, 530, 0, 4, false, 0),
    (1, 4, 356, 0, 3, false, 0),
    (2, 4, 530, 0, 4, true, 0),
    (3, 4, 0, 0, 1, false, 0),
    (0, 3, 530, 0, 4, true, 2),
    (1, 3, 920, 0, 3, false, 0),
    (2, 3, 530, 0, 4, false, 2),
    (3, 3, 0, 0, 1, false, 0),
    (0, 2, 0, 0, 14, true, 3),
    (1, 2, 211, 0, 4, false, 3),
    (2, 2, 0, 0, 14, false, 3),
    (3, 2, 0, 0, 14, false, 3),
    (0, 1, 0, 0, 14, false, 3),
    (1, 1, 0, 0, 14, false, 0),
    (2, 1, 0, 0, 14, true, 3),
    (3, 1, 724, 0, 3, false, 0),
    (0, 0, 330, 0, 3, false, 0),
    (1, 0, 0, 0, 14, false, 2),
    (2, 0, 0, 0, 14, true, 2),
    (3, 0, 0, 0, 1, false, 0),
];

// 1x3
const BROKEN_RAILS: [TILE ; 3] = [
    (0, 2, 299, 0, 3, true, 2),
    (0, 1, 331, 0, 3, false, 0),
    (0, 0, 299, 0, 3, false, 0),
];

// 5x6
const MUSH: [TILE ; 30] = [
    (0, 5, 458, 0, 12, false, 0),
    (1, 5, 0, 0, 6, false, 0),
    (2, 5, 0, 0, 3, false, 0),
    (3, 5, 0, 0, 3, true, 2),
    (4, 5, 0, 0, 6, false, 0),
    (0, 4, 0, 0, 1, false, 0),
    (1, 4, 0, 0, 3, false, 2),
    (2, 4, 0, 0, 3, false, 1),
    (3, 4, 0, 0, 3, false, 2),
    (4, 4, 458, 0, 12, true, 0),
    (0, 3, 0, 0, 1, false, 0),
    (1, 3, 0, 0, 3, false, 3),
    (2, 3, 844, 0, 3, false, 0),
    (3, 3, 0, 0, 3, false, 0),
    (4, 3, 0, 0, 3, true, 0),
    (0, 2, 844, 0, 4, false, 0),
    (1, 2, 0, 0, 3, false, 1),
    (2, 2, 0, 0, 3, true, 1),
    (3, 2, 0, 0, 3, true, 1),
    (4, 2, 0, 0, 3, false, 0),
    (0, 1, 0, 0, 1, false, 0),
    (1, 1, 458, 0, 12, false, 0),
    (2, 1, 0, 0, 3, true, 1),
    (3, 1, 0, 0, 3, true, 1),
    (4, 1, 0, 0, 6, true, 0),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 0, 0, 1, false, 0),
    (2, 0, 0, 0, 3, true, 1),
    (3, 0, 845, 0, 4, false, 0),
    (4, 0, 0, 0, 1, false, 0),
];

// 7x4
const CHARIOT: [TILE ; 28] = [
    (0, 3, 0, 0, 1, false, 0),
    (1, 3, 850, 0, 4, false, 0),
    (2, 3, 87, 0, 1, false, 0),
    (3, 3, 87, 0, 1, false, 0),
    (4, 3, 834, 0, 4, false, 2),
    (5, 3, 771, 0, 4, false, 3),
    (6, 3, 850, 0, 4, false, 1),
    (0, 2, 299, 0, 3, false, 1),
    (1, 2, 331, 4, 3, false, 3),
    (2, 2, 54, 4, 1, false, 0),
    (3, 2, 54, 4, 1, false, 2),
    (4, 2, 331, 4, 3, false, 3),
    (5, 2, 331, 4, 3, false, 3),
    (6, 2, 299, 0, 3, true, 1),
    (0, 1, 850, 0, 4, false, 3),
    (1, 1, 771, 0, 4, false, 1),
    (2, 1, 87, 4, 1, false, 2),
    (3, 1, 87, 4, 1, false, 2),
    (4, 1, 773, 0, 4, false, 2),
    (5, 1, 0, 0, 3, false, 0),
    (6, 1, 0, 0, 1, false, 0),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 0, 0, 1, false, 0),
    (2, 0, 724, 0, 4, false, 2),
    (3, 0, 834, 0, 4, false, 0),
    (4, 0, 724, 0, 4, false, 2),
    (5, 0, 0, 0, 1, false, 0),
    (6, 0, 0, 0, 1, false, 0),
];

// 2x4
const FLOWER: [TILE ; 8] = [
    (0, 3, 0, 0, 16, false, 2),
    (1, 3, 396, 0, 16, true, 2),
    (0, 2, 163, 0, 12, false, 2),
    (1, 2, 37, 0, 12, true, 2),
    (0, 1, 0, 0, 13, false, 0),
    (1, 1, 163, 0, 12, true, 2),
    (0, 0, 0, 0, 12, false, 0),
    (1, 0, 428, 0, 12, true, 2),
];

// 4x4
const FLOWER2: [TILE ; 16] = [
    (0, 3, 0, 0, 12, true, 0),
    (1, 3, 397, 0, 3, true, 0),
    (2, 3, 401, 0, 16, true, 2),
    (3, 3, 0, 0, 3, true, 0),
    (0, 2, 0, 0, 12, true, 0),
    (1, 2, 461, 0, 12, true, 0),
    (2, 2, 463, 0, 12, true, 0),
    (3, 2, 0, 0, 12, true, 1),
    (0, 1, 239, 0, 3, true, 0),
    (1, 1, 37, 0, 3, true, 2),
    (2, 1, 0, 0, 12, true, 0),
    (3, 1, 397, 0, 3, true, 2),
    (0, 0, 757, 0, 3, true, 0),
    (1, 0, 0, 0, 12, true, 0),
    (2, 0, 757, 0, 3, true, 2),
    (3, 0, 0, 0, 1, false, 0),
];

// 2x3
const GRASS: [TILE ; 6] = [
    (0, 2, 401, 0, 3, true, 0),
    (1, 2, 737, 0, 3, true, 0),
    (0, 1, 463, 0, 12, true, 0),
    (1, 1, 878, 0, 3, false, 2),
    (0, 0, 0, 0, 4, false, 2),
    (1, 0, 397, 0, 3, true, 0),
];

// 5x5
const GRASS2: [TILE ; 25] = [
    (0, 4, 0, 0, 1, false, 0),
    (1, 4, 0, 0, 1, false, 0),
    (2, 4, 0, 0, 1, false, 0),
    (3, 4, 75, 0, 3, true, 2),
    (4, 4, 0, 0, 1, false, 0),
    (0, 3, 75, 0, 3, true, 3),
    (1, 3, 11, 0, 3, true, 0),
    (2, 3, 13, 0, 3, true, 0),
    (3, 3, 43, 0, 3, true, 2),
    (4, 3, 0, 0, 3, true, 0),
    (0, 2, 459, 0, 3, true, 1),
    (1, 2, 433, 0, 3, true, 0),
    (2, 2, 462, 0, 12, true, 0),
    (3, 2, 39, 0, 3, true, 1),
    (4, 2, 34, 0, 3, true, 1),
    (0, 1, 75, 0, 3, true, 0),
    (1, 1, 43, 0, 3, true, 0),
    (2, 1, 459, 0, 3, true, 0),
    (3, 1, 13, 0, 3, true, 0),
    (4, 1, 928, 0, 3, true, 1),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 928, 0, 3, true, 0),
    (2, 0, 75, 0, 3, true, 0),
    (3, 0, 0, 0, 1, false, 0),
    (4, 0, 0, 0, 1, false, 0),
];

// 8x8
const LAVA_LAKE: [TILE ; 64] = [
    (0, 7, 396, 0, 4, false, 0),
    (1, 7, 0, 0, 1, false, 0),
    (2, 7, 0, 0, 1, false, 0),
    (3, 7, 710, 0, 4, false, 0),
    (4, 7, 712, 0, 4, false, 0),
    (5, 7, 715, 0, 4, false, 0),
    (6, 7, 0, 0, 1, false, 0),
    (7, 7, 0, 0, 1, false, 0),
    (0, 6, 0, 0, 1, false, 0),
    (1, 6, 0, 0, 0, false, 3),
    (2, 6, 161, 0, 4, false, 0),
    (3, 6, 430, 3, 4, false, 3),
    (4, 6, 163, 4, 3, false, 3),
    (5, 6, 430, 3, 4, false, 1),
    (6, 6, 174, 0, 4, false, 0),
    (7, 6, 0, 0, 1, false, 0),
    (0, 5, 0, 0, 1, false, 0),
    (1, 5, 463, 0, 4, false, 3),
    (2, 5, 161, 3, 4, false, 2),
    (3, 5, 239, 3, 2, false, 3),
    (4, 5, 10, 3, 2, false, 0),
    (5, 5, 163, 4, 3, false, 0),
    (6, 5, 161, 0, 4, false, 1),
    (7, 5, 0, 0, 1, false, 0),
    (0, 4, 715, 0, 4, false, 3),
    (1, 4, 200, 3, 4, false, 3),
    (2, 4, 647, 3, 2, false, 0),
    (3, 4, 773, 3, 2, true, 1),
    (4, 4, 162, 14, 2, true, 3),
    (5, 4, 803, 3, 2, true, 3),
    (6, 4, 169, 3, 4, true, 1),
    (7, 4, 725, 0, 4, false, 0),
    (0, 3, 710, 0, 4, false, 3),
    (1, 3, 163, 4, 3, false, 2),
    (2, 3, 239, 3, 2, true, 1),
    (3, 3, 328, 14, 2, true, 0),
    (4, 3, 233, 3, 2, true, 0),
    (5, 3, 168, 4, 3, true, 3),
    (6, 3, 169, 3, 4, true, 3),
    (7, 3, 239, 0, 4, true, 3),
    (0, 2, 458, 0, 4, false, 3),
    (1, 2, 429, 3, 4, false, 2),
    (2, 2, 201, 4, 3, false, 1),
    (3, 2, 234, 3, 2, true, 0),
    (4, 2, 231, 3, 2, true, 2),
    (5, 2, 169, 4, 3, true, 2),
    (6, 2, 8, 0, 4, true, 3),
    (7, 2, 0, 0, 7, true, 0),
    (0, 1, 0, 0, 1, false, 0),
    (1, 1, 161, 0, 4, false, 3),
    (2, 1, 163, 4, 3, false, 2),
    (3, 1, 169, 4, 3, false, 2),
    (4, 1, 169, 4, 3, false, 1),
    (5, 1, 163, 3, 4, false, 3),
    (6, 1, 239, 0, 4, false, 1),
    (7, 1, 11, 0, 4, false, 2),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 0, 0, 1, false, 0),
    (2, 0, 757, 0, 4, false, 0),
    (3, 0, 463, 0, 4, false, 2),
    (4, 0, 233, 0, 4, false, 3),
    (5, 0, 710, 0, 4, false, 2),
    (6, 0, 0, 0, 1, false, 0),
    (7, 0, 0, 0, 1, false, 0),
];

// 7x3
const CLOUD: [TILE ; 21] = [
    (0, 2, 0, 0, 1, false, 0),
    (1, 2, 0, 0, 3, false, 2),
    (2, 2, 234, 0, 3, false, 2),
    (3, 2, 711, 0, 3, false, 0),
    (4, 2, 234, 0, 3, true, 2),
    (5, 2, 174, 0, 3, true, 3),
    (6, 2, 0, 0, 1, false, 0),
    (0, 1, 430, 0, 3, false, 1),
    (1, 1, 260, 0, 3, true, 1),
    (2, 1, 324, 0, 3, true, 2),
    (3, 1, 395, 0, 3, false, 3),
    (4, 1, 325, 0, 3, false, 0),
    (5, 1, 429, 0, 3, false, 3),
    (6, 1, 261, 0, 3, true, 2),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 715, 0, 3, false, 2),
    (2, 0, 233, 0, 3, false, 3),
    (3, 0, 778, 0, 3, false, 2),
    (4, 0, 710, 0, 3, false, 2),
    (5, 0, 206, 0, 3, false, 0),
    (6, 0, 0, 0, 1, false, 0),
];

// 6x4
const ROCK: [TILE ; 24] = [
    (0, 3, 0, 0, 1, false, 0),
    (1, 3, 433, 0, 4, false, 1),
    (2, 3, 207, 0, 4, true, 2),
    (3, 3, 239, 0, 4, true, 0),
    (4, 3, 329, 0, 4, true, 2),
    (5, 3, 401, 0, 4, false, 1),
    (0, 2, 239, 0, 4, true, 1),
    (1, 2, 201, 0, 4, true, 3),
    (2, 2, 203, 0, 4, false, 2),
    (3, 2, 168, 0, 4, true, 1),
    (4, 2, 41, 0, 4, false, 2),
    (5, 2, 163, 0, 4, true, 3),
    (0, 1, 0, 0, 1, false, 0),
    (1, 1, 207, 0, 4, true, 0),
    (2, 1, 395, 0, 4, true, 3),
    (3, 1, 202, 0, 4, true, 0),
    (4, 1, 169, 0, 4, true, 0),
    (5, 1, 235, 0, 4, true, 0),
    (0, 0, 397, 0, 4, false, 1),
    (1, 0, 0, 0, 1, false, 0),
    (2, 0, 0, 0, 1, false, 0),
    (3, 0, 238, 0, 4, true, 0),
    (4, 0, 206, 0, 4, true, 0),
    (5, 0, 0, 0, 4, false, 2),
];

// 2x3
const GROUND: [TILE ; 6] = [
    (0, 2, 928, 0, 3, false, 0),
    (1, 2, 38, 0, 3, false, 0),
    (0, 1, 11, 0, 3, false, 0),
    (1, 1, 77, 0, 3, false, 2),
    (0, 0, 0, 0, 4, false, 2),
    (1, 0, 928, 0, 3, false, 2),
];

// 2x3
const GROUND2: [TILE ; 6] = [
    (0, 2, 878, 0, 3, false, 0),
    (1, 2, 932, 0, 3, false, 0),
    (0, 1, 11, 0, 3, false, 0),
    (1, 1, 871, 0, 3, false, 0),
    (0, 0, 0, 0, 4, false, 2),
    (1, 0, 878, 0, 3, false, 0),
];

// 2x3
const GROUND3: [TILE; 6] = [
    (0, 2, 878, 0, 3, false, 0),
    (1, 2, 891, 0, 3, false, 3),
    (0, 1, 878, 0, 3, false, 1),
    (1, 1, 878, 0, 3, false, 2),
    (0, 0, 0, 0, 4, false, 2),
    (1, 0, 757, 0, 3, true, 0),
];

// 2x2
const GROUND4: [TILE ; 4] = [
    (0, 1, 0, 0, 12, true, 0),
    (1, 1, 397, 0, 3, true, 2),
    (0, 0, 757, 0, 3, true, 2),
    (1, 0, 0, 0, 1, false, 0),
];

// 7x11
const LAVA: [TILE ; 77] = [
    (0, 10, 0, 0, 1, false, 0),
    (1, 10, 0, 0, 1, false, 0),
    (2, 10, 0, 0, 1, false, 0),
    (3, 10, 0, 0, 1, false, 0),
    (4, 10, 0, 0, 1, false, 0),
    (5, 10, 0, 0, 1, false, 0),
    (6, 10, 0, 0, 1, false, 0),
    (0, 9, 0, 0, 1, false, 0),
    (1, 9, 0, 0, 1, false, 0),
    (2, 9, 0, 0, 1, false, 0),
    (3, 9, 0, 0, 1, false, 0),
    (4, 9, 0, 0, 3, false, 0),
    (5, 9, 0, 0, 1, false, 0),
    (6, 9, 0, 0, 1, false, 0),
    (0, 8, 0, 0, 3, true, 1),
    (1, 8, 293, 0, 2, true, 0),
    (2, 8, 173, 0, 4, true, 0),
    (3, 8, 173, 0, 2, true, 0),
    (4, 8, 225, 0, 2, false, 2),
    (5, 8, 0, 0, 3, false, 1),
    (6, 8, 0, 0, 2, false, 0),
    (0, 7, 0, 0, 3, false, 3),
    (1, 7, 234, 0, 4, false, 2),
    (2, 7, 129, 4, 2, false, 3),
    (3, 7, 1023, 0, 2, false, 3),
    (4, 7, 1023, 0, 2, false, 3),
    (5, 7, 225, 0, 2, true, 1),
    (6, 7, 0, 0, 3, false, 0),
    (0, 6, 205, 0, 4, false, 0),
    (1, 6, 163, 4, 2, false, 3),
    (2, 6, 1023, 0, 2, false, 2),
    (3, 6, 162, 14, 2, false, 3),
    (4, 6, 1023, 0, 2, false, 2),
    (5, 6, 165, 4, 2, false, 0),
    (6, 6, 239, 0, 2, false, 1),
    (0, 5, 205, 0, 4, false, 0),
    (1, 5, 225, 4, 2, false, 1),
    (2, 5, 168, 14, 2, false, 3),
    (3, 5, 168, 14, 2, true, 0),
    (4, 5, 1023, 0, 2, false, 2),
    (5, 5, 1023, 0, 2, false, 2),
    (6, 5, 8, 0, 2, false, 2),
    (0, 4, 0, 0, 1, false, 0),
    (1, 4, 234, 0, 2, false, 1),
    (2, 4, 200, 14, 2, true, 1),
    (3, 4, 163, 14, 2, false, 2),
    (4, 4, 1023, 0, 2, false, 2),
    (5, 4, 235, 4, 2, false, 3),
    (6, 4, 205, 0, 4, false, 2),
    (0, 3, 293, 0, 2, true, 3),
    (1, 3, 234, 0, 2, false, 2),
    (2, 3, 1023, 0, 2, false, 2),
    (3, 3, 168, 14, 2, false, 1),
    (4, 3, 162, 14, 2, false, 1),
    (5, 3, 174, 4, 2, false, 0),
    (6, 3, 224, 0, 4, false, 3),
    (0, 2, 233, 0, 2, true, 2),
    (1, 2, 168, 14, 2, true, 3),
    (2, 2, 1023, 0, 2, false, 2),
    (3, 2, 234, 4, 2, false, 0),
    (4, 2, 234, 4, 2, true, 0),
    (5, 2, 163, 4, 2, false, 0),
    (6, 2, 239, 0, 4, false, 1),
    (0, 1, 235, 0, 2, false, 0),
    (1, 1, 233, 0, 2, false, 3),
    (2, 1, 271, 0, 2, false, 3),
    (3, 1, 360, 0, 4, true, 0),
    (4, 1, 224, 0, 4, true, 0),
    (5, 1, 163, 4, 2, false, 2),
    (6, 1, 99, 0, 2, false, 3),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 0, 0, 1, false, 0),
    (2, 0, 0, 0, 1, false, 0),
    (3, 0, 0, 0, 1, false, 0),
    (4, 0, 0, 0, 1, false, 0),
    (5, 0, 206, 0, 4, false, 0),
    (6, 0, 0, 0, 1, false, 0),
];

// 7x11
const LAVA2: [TILE ; 77] = [
    (0, 10, 0, 0, 1, false, 0),
    (1, 10, 0, 0, 1, false, 0),
    (2, 10, 0, 0, 1, false, 0),
    (3, 10, 0, 0, 1, false, 0),
    (4, 10, 0, 0, 1, false, 0),
    (5, 10, 0, 0, 1, false, 0),
    (6, 10, 0, 0, 1, false, 0),
    (0, 9, 0, 0, 1, false, 0),
    (1, 9, 0, 0, 1, false, 0),
    (2, 9, 0, 0, 1, false, 0),
    (3, 9, 0, 0, 1, false, 0),
    (4, 9, 0, 0, 3, false, 0),
    (5, 9, 0, 0, 1, false, 0),
    (6, 9, 0, 0, 1, false, 0),
    (0, 8, 0, 0, 3, true, 1),
    (1, 8, 293, 0, 2, true, 0),
    (2, 8, 173, 0, 4, true, 0),
    (3, 8, 173, 0, 2, true, 0),
    (4, 8, 225, 0, 2, false, 2),
    (5, 8, 0, 0, 3, false, 1),
    (6, 8, 0, 0, 2, false, 0),
    (0, 7, 0, 0, 3, false, 3),
    (1, 7, 234, 0, 4, false, 2),
    (2, 7, 129, 4, 2, false, 3),
    (3, 7, 1023, 0, 2, false, 3),
    (4, 7, 1023, 0, 2, false, 3),
    (5, 7, 225, 0, 2, true, 1),
    (6, 7, 0, 0, 3, false, 0),
    (0, 6, 205, 0, 4, false, 0),
    (1, 6, 163, 4, 2, false, 3),
    (2, 6, 1023, 0, 2, false, 2),
    (3, 6, 162, 14, 2, false, 3),
    (4, 6, 1023, 0, 2, false, 2),
    (5, 6, 165, 4, 2, false, 0),
    (6, 6, 239, 0, 2, false, 1),
    (0, 5, 205, 0, 4, false, 0),
    (1, 5, 225, 4, 2, false, 1),
    (2, 5, 168, 14, 2, false, 3),
    (3, 5, 168, 14, 2, true, 0),
    (4, 5, 1023, 0, 2, false, 2),
    (5, 5, 1023, 0, 2, false, 2),
    (6, 5, 8, 0, 2, false, 2),
    (0, 4, 0, 0, 1, false, 0),
    (1, 4, 234, 0, 2, false, 1),
    (2, 4, 200, 14, 2, true, 1),
    (3, 4, 163, 14, 2, false, 2),
    (4, 4, 1023, 0, 2, false, 2),
    (5, 4, 235, 4, 2, false, 3),
    (6, 4, 205, 0, 4, false, 2),
    (0, 3, 293, 0, 2, true, 3),
    (1, 3, 234, 0, 2, false, 2),
    (2, 3, 1023, 0, 2, false, 2),
    (3, 3, 168, 14, 2, false, 1),
    (4, 3, 162, 14, 2, false, 1),
    (5, 3, 174, 4, 2, false, 0),
    (6, 3, 224, 0, 4, false, 3),
    (0, 2, 233, 0, 2, true, 2),
    (1, 2, 168, 14, 2, true, 3),
    (2, 2, 1023, 0, 2, false, 2),
    (3, 2, 234, 4, 2, false, 0),
    (4, 2, 234, 4, 2, true, 0),
    (5, 2, 163, 4, 2, false, 0),
    (6, 2, 239, 0, 4, false, 1),
    (0, 1, 235, 0, 2, false, 0),
    (1, 1, 233, 0, 2, false, 3),
    (2, 1, 271, 0, 2, false, 3),
    (3, 1, 360, 0, 4, true, 0),
    (4, 1, 224, 0, 4, true, 0),
    (5, 1, 163, 4, 2, false, 2),
    (6, 1, 99, 0, 2, false, 3),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 0, 0, 1, false, 0),
    (2, 0, 0, 0, 1, false, 0),
    (3, 0, 0, 0, 1, false, 0),
    (4, 0, 0, 0, 1, false, 0),
    (5, 0, 206, 0, 4, false, 0),
    (6, 0, 0, 0, 1, false, 0),
];

// 5x4
const ROCKS: [TILE ; 20] = [
    (0, 3, 0, 0, 1, false, 0),
    (1, 3, 0, 0, 1, false, 0),
    (2, 3, 0, 0, 1, false, 0),
    (3, 3, 0, 0, 1, false, 0),
    (4, 3, 323, 0, 4, false, 0),
    (0, 2, 396, 0, 4, false, 0),
    (1, 2, 0, 0, 1, false, 0),
    (2, 2, 0, 0, 12, false, 0),
    (3, 2, 0, 0, 1, false, 0),
    (4, 2, 0, 0, 1, false, 0),
    (0, 1, 0, 0, 1, false, 0),
    (1, 1, 0, 0, 1, false, 0),
    (2, 1, 0, 0, 1, false, 0),
    (3, 1, 0, 0, 1, false, 0),
    (4, 1, 0, 0, 1, false, 0),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 11, 0, 4, false, 2),
    (2, 0, 0, 0, 1, false, 0),
    (3, 0, 0, 0, 1, false, 0),
    (4, 0, 0, 0, 1, false, 0),
];