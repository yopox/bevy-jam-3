use bevy::app::{App, Plugin};
use bevy::prelude::{Component, Query, Transform};
use bevy::utils::default;
use strum::IntoEnumIterator;

use crate::graphics::monsters::Monsters;
use crate::graphics::sprites;
use crate::graphics::sprites::TILE;
use crate::util::Palette;
use crate::weapons::{Weapon, Weapons};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(collide);
    }
}

#[derive(Component, Default)]
pub struct Hitbox {
    pub dx: f32,
    pub dy: f32,
    pub width: f32,
    pub height: f32,
}

impl Hitbox {
    /// Returns the collider for the tile [index]
    pub fn for_tile(index: usize, transparent_bg: bool) -> Option<Hitbox> {
        match (index, transparent_bg) {
            _ => Some(Hitbox { width: 8.0, height: 8.0, ..default() }),
            // (0, false) => Some(Hitbox { width: 8.0, height: 8.0, ..default() }),
            // _ => None,
        }
    }

    // TODO: formula to update hitbox based on the tile [flip] and [rotation] properties.
    pub fn with_flip_and_rotation(&self, flip: bool, rotation: u8) -> Self {
        Hitbox {
            dx: self.dx,
            dy: self.dy,
            width: self.width,
            height: self.height,
        }
    }
}

pub fn collide(
    colliders: Query<(&Hitbox, &Transform)>
) {
    // TODO: Use [/Users/yopox/.cargo/registry/src/github.com-1ecc6299db9ec823/bevy_sprite-0.10.1/src/collide_aabb.rs:24]
}

#[test]
fn sprites_have_hitbox() {
    let has_hitbox = |sprite: &[TILE]| {
        sprite
            .iter()
            .find(|(_, _, index, bg, _, _, _)| Hitbox::for_tile(*index, *bg == 0).is_some())
            .is_some()
    };

    assert!(has_hitbox(&sprites::SHIP), "The ship has no hitbox!");

    for monster in Monsters::iter() {
        assert!(has_hitbox(monster.sprite()), "The monster {:?} has no hitbox!", monster)
    }

    for weapon in Weapons::iter() {
        let w: Weapon = weapon.into();
        assert!(Hitbox::for_tile(w.shot_tile.index, w.shot_tile.bg == Palette::Transparent).is_some(), "The weapon {:?} has no hitbox!", weapon)
    }
}