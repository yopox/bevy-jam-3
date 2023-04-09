use bevy::hierarchy::{Children, HierarchyQueryExt};
use bevy::prelude::{Added, Commands, Component, Entity, Handle, Query, Transform};
use bevy::sprite::TextureAtlas;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};

use crate::collision::SolidBody;
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
            bg: Palette::Transparent, fg: Palette::Transparent,
            flip, rotation
        }
    }

    pub fn from_index(index: usize) -> Self {
        Tile {
            index,
            bg: Palette::Transparent, fg: Palette::Transparent,
            flip: false, rotation: 0
        }
    }

    pub fn with_fg(&mut self, fg: Palette) -> Self {
        Tile { index: self.index, bg: self.bg, fg, flip: self.flip, rotation: self.rotation, }
    }

    pub fn with_rotation(&mut self, rotation: u8) -> Self {
        Tile { index: self.index, bg: self.bg, fg: self.fg, flip: self.flip, rotation, }
    }

    pub fn flip(&mut self) -> Self {
        Tile { index: self.index, bg: self.bg, fg: self.fg, flip: !self.flip, rotation: self.rotation, }
    }

    pub fn sprite(&self, x: usize, y: usize, z: f32, atlas: &Handle<TextureAtlas>) -> TextModeSpriteSheetBundle {
        sprite(self.index, x, y, z, self.bg, self.fg, self.flip, self.rotation, atlas.clone())
    }
}

pub enum Tiles {
    LeftHand,
    Dash,
    DoubleCannon,
    Dot,
}

impl Tiles {
    pub fn to_tile(&self) -> Tile {
        match self {
            Tiles::LeftHand => Tile::from_index(718),
            Tiles::Dash => Tile::from_index(877),
            Tiles::DoubleCannon => Tile::from_index(1021).with_rotation(1),
            Tiles::Dot => Tile::from_index(860),
        }
    }
}

#[derive(Component)]
pub struct Flip;

pub fn flip(
    mut commands: Commands,
    flipped: Query<(&SolidBody, Entity), Added<Flip>>,
    children_query: Query<&Children>,
    mut tile: Query<(&mut TextModeTextureAtlasSprite, &mut Transform)>
) {
    for (body, id) in &flipped {
        for child in children_query.iter_descendants(id) {
            let Ok((mut sprite, mut transform)) = tile.get_mut(child) else { continue };
            sprite.flip_x = !sprite.flip_x;
            transform.translation.x = body.width - transform.translation.x - 8.;
        }
        commands.entity(id).remove::<Flip>();
    }
}

