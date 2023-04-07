pub type X = usize;
pub type Y = usize;
pub type INDEX = usize;
pub type BG = usize;
pub type FG = usize;
pub type FLIP = bool;
pub type ROTATION = u8;
pub type TILE = (X, Y, INDEX, BG, FG, FLIP, ROTATION);

pub const SHIP: [TILE; 20] = [
    (0, 4, 0, 0, 1, false, 0),
    (1, 4, 1010, 0, 2, false, 0),
    (2, 4, 1010, 0, 2, true, 0),
    (3, 4, 0, 0, 1, false, 0),
    (0, 3, 62, 1, 0, false, 2),
    (1, 3, 605, 0, 1, true, 0),
    (2, 3, 605, 0, 1, false, 0),
    (3, 3, 62, 1, 0, true, 2),
    (0, 2, 337, 0, 1, false, 0),
    (1, 2, 510, 1, 3, false, 0),
    (2, 2, 510, 1, 3, true, 0),
    (3, 2, 337, 0, 1, true, 0),
    (0, 1, 62, 1, 0, true, 0),
    (1, 1, 605, 0, 1, false, 2),
    (2, 1, 605, 0, 1, true, 2),
    (3, 1, 62, 1, 0, false, 0),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 1010, 0, 2, true, 2),
    (2, 0, 1010, 0, 2, false, 2),
    (3, 0, 0, 0, 1, false, 0),
];

pub const STAR_FLY: [TILE; 9] = [
    (0, 2, 0, 0, 1, false, 0),
    (1, 2, 927, 0, 1, true, 0),
    (2, 2, 0, 0, 1, false, 0),
    (0, 1, 487, 0, 2, false, 0),
    (1, 1, 874, 1, 2, false, 0),
    (2, 1, 487, 0, 2, true, 0),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 835, 0, 1, true, 0),
    (2, 0, 0, 0, 1, false, 0),
];

pub const SUPER_EYE: [TILE; 25] = [
    (0, 4, 0, 0, 1, false, 0),
    (1, 4, 624, 0, 1, false, 1),
    (2, 4, 149, 0, 1, false, 0),
    (3, 4, 624, 0, 1, true, 1),
    (4, 4, 0, 0, 1, false, 1),
    (0, 3, 624, 0, 1, true, 2),
    (1, 3, 734, 0, 1, false, 0),
    (2, 3, 859, 0, 1, false, 0),
    (3, 3, 734, 0, 1, false, 1),
    (4, 3, 624, 0, 1, false, 2),
    (0, 2, 149, 0, 1, false, 0),
    (1, 2, 859, 0, 1, false, 3),
    (2, 2, 644, 0, 2, false, 0),
    (3, 2, 859, 0, 1, false, 1),
    (4, 2, 149, 0, 1, false, 0),
    (0, 1, 624, 0, 1, false, 0),
    (1, 1, 734, 0, 1, false, 3),
    (2, 1, 859, 0, 1, false, 2),
    (3, 1, 734, 0, 1, false, 2),
    (4, 1, 624, 0, 1, true, 0),
    (0, 0, 0, 0, 1, false, 3),
    (1, 0, 624, 0, 1, true, 3),
    (2, 0, 149, 0, 1, false, 0),
    (3, 0, 624, 0, 1, false, 3),
    (4, 0, 0, 0, 1, false, 2),
];

pub const SPACE_SHRIMP: [TILE; 9] = [
    (0, 2, 0, 0, 1, false, 0),
    (1, 2, 670, 0, 2, false, 0),
    (2, 2, 744, 0, 1, false, 3),
    (0, 1, 0, 0, 2, true, 2),
    (1, 1, 712, 0, 2, true, 1),
    (2, 1, 463, 0, 2, false, 1),
    (0, 0, 489, 0, 2, false, 0),
    (1, 0, 701, 0, 2, false, 0),
    (2, 0, 0, 0, 1, false, 0),
];

pub const CASH_KNIGHT: [TILE; 9] = [
    (0, 2, 0, 0, 1, false, 0),
    (1, 2, 939, 0, 2, false, 0),
    (2, 2, 0, 0, 1, false, 0),
    (0, 1, 868, 0, 2, false, 0),
    (1, 1, 228, 0, 1, false, 0),
    (2, 1, 941, 0, 2, false, 1),
    (0, 0, 343, 0, 1, true, 0),
    (1, 0, 206, 0, 1, false, 0),
    (2, 0, 343, 0, 1, false, 0),
];

pub const SPACE_CRAB: [TILE; 12] = [
    (0, 2, 0, 0, 1, false, 0),
    (1, 2, 967, 0, 2, false, 2),
    (2, 2, 967, 0, 2, false, 2),
    (3, 2, 0, 0, 1, false, 0),
    (0, 1, 619, 0, 2, true, 0),
    (1, 1, 134, 1, 2, false, 0),
    (2, 1, 134, 1, 2, true, 0),
    (3, 1, 619, 0, 2, false, 0),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 834, 0, 2, true, 0),
    (2, 0, 834, 0, 2, false, 0),
    (3, 0, 0, 0, 1, false, 0),
];

pub const MAGIC_CANDLE: [TILE; 6] = [
    (0, 1, 0, 0, 1, false, 0),
    (1, 1, 268, 0, 3, true, 0),
    (2, 1, 0, 0, 1, false, 0),
    (0, 0, 777, 0, 2, true, 0),
    (1, 0, 418, 0, 1, true, 0),
    (2, 0, 777, 0, 2, false, 0),
];

pub const NECROMANCER: [TILE; 9] = [
    (0, 2, 0, 0, 1, false, 0),
    (1, 2, 357, 0, 2, true, 0),
    (2, 2, 0, 0, 1, false, 0),
    (0, 1, 777, 0, 2, true, 0),
    (1, 1, 229, 0, 1, true, 0),
    (2, 1, 777, 0, 2, false, 0),
    (0, 0, 0, 0, 1, false, 0),
    (1, 0, 367, 0, 1, false, 1),
    (2, 0, 0, 0, 1, false, 0),
];