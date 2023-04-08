use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{Color, TextureAtlas, Transform};
use bevy::sprite::Anchor;
use bevy::utils::default;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};
use lazy_static::lazy_static;

use crate::util::size::tile_to_f32;

pub mod size {
    pub const SCALE: f32 = 5.;
    pub(in crate::util) const TILE_SIZE: f32 = 8.;
    pub const WIDTH: usize = 32;
    pub const HEIGHT: usize = 18;

    /// Returns world coordinates for a tile, for instance `2` -> `2 as f32 * TILE_SIZE`.
    pub fn tile_to_f32(tile: usize) -> f32 { tile as f32 * TILE_SIZE }
}

pub mod z_pos {
    pub const BACKGROUND: f32 = 0.;
    pub const ENEMIES: f32 = 2.;
    pub const SHOTS: f32 = 4.;
    pub const RAILS: f32 = 6.;
    pub const MACHINE: f32 = 8.;
    pub const FRAME: f32 = 10.;
    pub const GUI: f32 = 12.;
}

#[derive(Copy, Clone)]
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
    pos.x < -8. || pos.x > tile_to_f32(size::WIDTH) + 8. || pos.y < -8. || pos.y > tile_to_f32(size::HEIGHT) + 8.
}

pub mod ship {
    use crate::util::size;

    pub const SPEED: f32 = 0.3;
    pub const INIT_Y: f32 = size::HEIGHT as f32 * size::TILE_SIZE / 2. - 16.;
    pub const MAX_Y: i64 = 190;
    pub const MIN_Y: i64 = -170;
}