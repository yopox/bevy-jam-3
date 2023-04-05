use bevy::prelude::*;

use crate::GameState;
use crate::graphics::ship::Ship;
use crate::graphics::tiles::{Tile, Tiles};
use crate::util::Palette;

/// Kind of shots
enum Shots {
    /// Straight line, 1 tile
    Simple,
    /// 2 shots, diagonal (45 deg)
    Double,
    /// Fill line (until obstacle) + follow player
    Laser,
}

/// Weapon description (left orientation)
struct Weapon {
    shots: Shots,
    tile: Tile,
    shot_tile: Tile,
    cooldown: u16,
}

pub enum Weapons {
    Finger,
}

impl Into<Weapon> for Weapons {
    fn into(self) -> Weapon {
        match self {
            Weapons::Finger => Weapon {
                shots: Shots::Simple,
                tile: Tiles::LeftHand.to_tile().with_fg(Palette::BLACK),
                shot_tile: Tiles::Dash.to_tile().with_fg(Palette::BLACK),
                cooldown: 10,
            }
        }
    }
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(
                update_weapons
                    .in_set(OnUpdate(GameState::Survival))
            );
    }
}

#[derive(Eq, PartialEq)]
pub enum Side { Left, Right }

#[derive(Component)]
pub struct ActiveWeapon(Side);

pub fn spawn_weapon(
    weapon: Weapons,
    side: Side,
    commands: &mut Commands,
    atlas: &Handle<TextureAtlas>
) {
    let mut weapon: Weapon = weapon.into();
    if side == Side::Right { weapon.tile.flip = !weapon.tile.flip };
    commands
        .spawn(ActiveWeapon(side))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(VisibilityBundle::default())
        .with_children(|spawn| { spawn.spawn(weapon.tile.sprite(0, 0, 0., atlas)); });
}

pub fn update_weapons(
    ship: Query<&Transform, With<Ship>>,
    mut weapons: Query<(&ActiveWeapon, &mut Transform), Without<Ship>>,
) {
    let ship_pos = ship.single().translation;

    for (weapon, mut pos) in weapons.iter_mut() {
        pos.translation.y = ship_pos.y + 16.;
        pos.translation.x = ship_pos.x + if weapon.0 == Side::Left { -8. } else { 32. };
    }
}