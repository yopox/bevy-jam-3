use bevy::asset::Handle;
use bevy::math::Vec3;
use bevy::prelude::{Color, TextureAtlas, Transform};
use bevy::sprite::Anchor;
use bevy::utils::default;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};

use crate::util::size::{HEIGHT, TILE_SIZE, WIDTH};

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
    BLACK,
    WHITE,
    GRAY,
    DARK_GRAY,
    RED,
    DARK_BLUE,
    TRANSPARENT,
}

impl Palette {
    // TODO: Don't parse colors each time
    pub fn color(&self) -> Color {
        match self {
            Palette::BLACK => Color::hex("#000000"),
            Palette::WHITE => Color::hex("#ffffff"),
            Palette::GRAY => Color::hex("#808080"),
            Palette::DARK_GRAY => Color::hex("#404040"),
            Palette::RED => Color::hex("#ff4500"),
            Palette::DARK_BLUE => Color::hex("#344866"),
            Palette::TRANSPARENT => Color::hex("#00000000"),
        }.unwrap()
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
            bg: bg.color(),
            fg: fg.color(),
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
    pos.x < -8. || pos.x > (WIDTH as f32 * TILE_SIZE) + 8. || pos.y < -8. || pos.y > (HEIGHT as f32 * TILE_SIZE) + 8.
}