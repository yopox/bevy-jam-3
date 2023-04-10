use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{Color, TextureAtlas, Transform};
use bevy::sprite::Anchor;
use bevy::utils::default;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};
use lazy_static::lazy_static;

use size::{HEIGHT, tile_to_f32, WIDTH};

pub mod size {
    pub const SCALE: f32 = 5.;

    const TILE_SIZE: usize = 8;

    pub const WIDTH: usize = 32;
    pub const HEIGHT: usize = 18;

    /// Returns world coordinates for a tile, for instance `2` -> `(2 * TILE_SIZE) as f32 `.
    pub const fn tile_to_f32(tile: usize) -> f32 { (tile * TILE_SIZE) as f32 }
}

pub mod z_pos {
    pub const BACKGROUND: f32 = 0.;
    pub const RAILS: f32 = 1.;
    pub const ENEMIES: f32 = 2.;
    pub const SHOTS: f32 = 4.;
    pub const WEAPONS: f32 = 7.;
    pub const MACHINE: f32 = 8.;
    pub const BACKGROUND_TEXT: f32 = 8.5;
    pub const TRANSITION: f32 = 9.;
    pub const FRAME: f32 = 10.;
    pub const GUI: f32 = 12.;
    pub const CHOOSE_BORDER: f32 = 14.;
}

pub mod fight {
    pub const ENEMY_COOLDOWN: usize = 120;
    pub const MONSTERS_FREEZE: usize = 40;
    pub const LASER_LOADING: usize = 30;
    pub const LASER_FIRING: usize = 45;
}

pub mod background {
    pub const SPEED: f32 = 0.1;
    pub const ALPHA: f32 = 0.35;
    pub const LAYOUT_WIDTH: usize = 13;
    pub const LAYOUT_HEIGHT: usize = 13;
}

pub mod misc {
    pub const ANIMATION_INTERVAL: usize = 80;
}

pub mod choose {
    pub const TEXT_SELECT: &'static str = " Press ← or → to select";
    pub const TEXT_CHOOSE_LEFT: &'static str = "Press ← again to confirm";
    pub const TEXT_CHOOSE_RIGHT: &'static str = "Press → again to confirm";
    pub const BORDER_WIDTH: usize = 11;
    pub const BORDER_HEIGHT: usize = 10;
    pub const BORDER_X: usize = 3;
    pub const BORDER_Y: usize = 4;
    pub const SIDE_Y: usize = 15;
}

pub mod transition {
    use crate::util::size::HEIGHT;

    pub const HALF_HEIGHT: usize = HEIGHT / 2 - 1;
    pub const SPEED: u64 = 800;
}

pub mod tweening {
    pub const TRANSITION_OVER: u64 = 1;
    pub const DELAY: u64 = 200;
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Palette {
    Black = 0,

    Blue,
    LightBlue,

    Gold,
    LightGold,

    Purple,
    LightPurple,

    Cactus,
    LightCactus,

    Terracotta,
    LightTerracotta,

    Red,
    LightRed,

    Lava,
    Gravel,
    Rock,
    Dark,

    Transparent,
}

lazy_static! {
    static ref COLOR_OF_PALETTE: [Color; 18] = [
        Color::hex("#000000").unwrap(),
        Color::hex("#627EAF").unwrap(),
        Color::hex("#B3CBDF").unwrap(),
        Color::hex("#FBCC0A").unwrap(),
        Color::hex("#FBEAA6").unwrap(),
        Color::hex("#88519B").unwrap(),
        Color::hex("#CBAAD1").unwrap(),
        Color::hex("#70824D").unwrap(),
        Color::hex("#B8BF9D").unwrap(),
        Color::hex("#C17329").unwrap(),
        Color::hex("#F7C8A5").unwrap(),
        Color::hex("#9E3636").unwrap(),
        Color::hex("#F4898B").unwrap(),
        Color::hex("#F7913D").unwrap(),
        Color::hex("#989281").unwrap(),
        Color::hex("#5B524C").unwrap(),
        Color::hex("#3D2F2C").unwrap(),
        Color::hex("#00000000").unwrap(),
    ];
}

impl Into<Color> for Palette {
    fn into(self) -> Color {
        COLOR_OF_PALETTE[self as usize]
    }
}

pub fn sprite(
    index: usize,
    x: usize, y: usize, z: f32,
    bg: Palette, fg: Palette,
    flip: bool, rotation: u8,
    atlas: Handle<TextureAtlas>,
) -> TextModeSpriteSheetBundle {
    TextModeSpriteSheetBundle {
        sprite: TextModeTextureAtlasSprite {
            bg: bg.into(),
            fg: fg.into(),
            alpha: 1.0,
            index,
            flip_x: flip,
            rotation,
            anchor: Anchor::BottomLeft,
            ..default()
        },
        texture_atlas: atlas,
        transform: Transform {
            translation: Vec3::new(tile_to_f32(x), tile_to_f32(y) , z),
            ..default()
        },
        ..default()
    }
}

pub fn is_oob(transform: &Transform) -> bool {
    let pos = transform.translation;
    pos.x < -8. || pos.x > tile_to_f32(WIDTH) + 8. || pos.y < -8. || pos.y > tile_to_f32(HEIGHT) + 8.
}

pub mod ship {
    use crate::util::size::{HEIGHT, tile_to_f32, WIDTH};

    pub const SPEED: f32 = 0.3;
    pub const INIT_Y: f32 = tile_to_f32(HEIGHT / 2 - 2);
    pub const MAX_Y: i64 = 190;
    pub const MIN_Y: i64 = -170;
    pub const LASER_LENGTH: usize = WIDTH / 2 - 3;
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Side { Left, Right }

impl Side {
    pub const fn to_sign_i8(self) -> i8 {
        match self {
            Side::Left => 1,
            Side::Right => -1,
        }
    }

    pub const fn to_sign_f32(self) -> f32 {
        match self {
            Side::Left => 1.,
            Side::Right => -1.,
        }
    }

    // Here Copy is just to ensure const. If we want to use it on non-Copy types, removing both annotations should be enough
    pub const fn on_left_right<T>(self, x_left: T, x_right: T) -> T where T: Copy {
        match self {
            Side::Left => x_left,
            Side::Right => x_right,
        }
    }

    pub fn of_x(x: usize) -> Self {
        if x < WIDTH / 2 { Side::Left } else { Side::Right }
    }

    pub const fn flip(self) -> Self {
        match self {
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }
}
