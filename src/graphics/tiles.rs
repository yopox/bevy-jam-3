use bevy::prelude::Handle;
use bevy::sprite::TextureAtlas;
use bevy_text_mode::TextModeSpriteSheetBundle;

use crate::util::{Palette, sprite};

#[derive(Copy, Clone)]
pub struct Tile {
    pub index: usize,
    pub bg: Palette,
    pub fg: Palette,
    pub flip: bool,
    pub rotation: u8,
}

impl Tile {
    pub fn new(index: usize, flip: bool, rotation: u8) -> Self {
        Tile {
            index,
            bg: Palette::TRANSPARENT, fg: Palette::TRANSPARENT,
            flip, rotation
        }
    }

    pub fn from_index(index: usize) -> Self {
        Tile {
            index,
            bg: Palette::TRANSPARENT, fg: Palette::TRANSPARENT,
            flip: false, rotation: 0
        }
    }

    pub fn with_fg(&mut self, fg: Palette) -> Self {
        Tile {
            index: self.index,
            bg: self.bg,
            fg,
            flip: self.flip,
            rotation: self.rotation,
        }
    }

    pub fn sprite(&self, x: usize, y: usize, z: f32, atlas: &Handle<TextureAtlas>) -> TextModeSpriteSheetBundle {
        sprite(self.index, x, y, z, self.bg, self.fg, self.flip, self.rotation, atlas.clone())
    }
}

pub enum Tiles {
    LeftHand,
    Dash,
}

impl Tiles {
    pub fn to_tile(&self) -> Tile {
        match self {
            Tiles::LeftHand => Tile::from_index(718),
            Tiles::Dash => Tile::from_index(877),
        }
    }
}