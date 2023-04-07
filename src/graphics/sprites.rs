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