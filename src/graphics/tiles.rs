use bevy::hierarchy::{Children, HierarchyQueryExt};
use bevy::prelude::{Added, Commands, Component, Entity, Handle, Query, Transform};
use bevy::sprite::TextureAtlas;
use bevy_text_mode::{TextModeSpriteSheetBundle, TextModeTextureAtlasSprite};

use crate::collision::SolidBody;
use crate::graphics::sprites::ROTATION;
use crate::util::{Palette, sprite};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum Rotation {
    #[default]
    No = 0,
    Right,
    Flip,
    Left,
}

impl Into<ROTATION> for Rotation {
    fn into(self) -> ROTATION {
        self as ROTATION
    }
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub index: usize,
    pub bg: Palette,
    pub fg: Palette,
    pub flip: bool,
    pub rotation: Rotation,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            index: 0,
            bg: Palette::Transparent,
            fg: Palette::Transparent,
            flip: false,
            rotation: Rotation::default(),
        }
    }
}

impl Tile {
    pub fn new(index: usize, flip: bool, rotation: Rotation) -> Self {
        Tile { index, flip, rotation, ..Tile::default() }
    }

    pub fn from_index(index: usize) -> Self {
        Tile { index, ..Tile::default() }
    }

    pub fn with_fg(self, fg: Palette) -> Self {
        Tile { fg, ..self }
    }

    pub fn with_rotation(self, rotation: Rotation) -> Self {
        Tile { rotation, ..self }
    }

    pub fn flip(self) -> Self {
        Tile { flip: !self.flip, ..self }
    }

    pub fn sprite(&self, x: usize, y: usize, z: f32, atlas: &Handle<TextureAtlas>) -> TextModeSpriteSheetBundle {
        sprite(self.index, x, y, z, self.bg, self.fg, self.flip, self.rotation.into(), atlas.clone())
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
            Tiles::DoubleCannon => Tile::from_index(1021).with_rotation(Rotation::Right),
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

