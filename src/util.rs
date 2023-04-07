use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{Color, TextureAtlas, Transform};
use bevy::sprite::Anchor;
use bevy::utils::default;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};
use lazy_static::lazy_static;

pub mod size {
    pub const SCALE: f32 = 5.;
    pub const TILE_SIZE: f32 = 8.;
    pub const WIDTH: usize = 32;
    pub const HEIGHT: usize = 18;
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
    White,
    Gray,
    DarkGray,
    Red,
    DarkBlue,
    Transparent,
}

lazy_static! {
    static ref COLOR_OF_PALETTE: [Color; 7] = [
        Color::hex("#000000").unwrap(),
        Color::hex("#ffffff").unwrap(),
        Color::hex("#808080").unwrap(),
        Color::hex("#404040").unwrap(),
        Color::hex("#ff4500").unwrap(),
        Color::hex("#344866").unwrap(),
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
            translation: Vec3::new(x as f32 * size::TILE_SIZE, y as f32 * size::TILE_SIZE, z),
            ..default()
        },
        ..default()
    }
}

pub fn is_oob(transform: &Transform) -> bool {
    let pos = transform.translation;
    pos.x < -8. || pos.x > (size::WIDTH as f32 * size::TILE_SIZE) + 8. || pos.y < -8. || pos.y > (size::HEIGHT as f32 * size::TILE_SIZE) + 8.
}

pub const SHIP_SPEED: f32 = 0.3;
pub const SHIP_INIT_Y: f32 = size::HEIGHT as f32 * size::TILE_SIZE / 2. - 16.;
pub const SHIP_MAX_Y: i64 = 190;
pub const SHIP_MIN_Y: i64 = -170;
