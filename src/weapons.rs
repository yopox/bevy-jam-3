use bevy::prelude::*;

use crate::GameState;
use crate::graphics::ship::Ship;
use crate::graphics::tiles::{Tile, Tiles};
use crate::util::Palette;

/// Kind of shots
#[derive(Copy, Clone)]
enum Shots {
    /// Straight line, 1 tile
    Simple,
    /// 2 shots, diagonal (45 deg)
    Double,
    /// Fill line (until obstacle) + follow player
    Laser,
}

/// Weapon description (left orientation)
#[derive(Copy, Clone)]
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
                cooldown: 40,
            }
        }
    }
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(
                (update_weapons, shoot)
                    .in_set(OnUpdate(GameState::Survival))
            );
    }
}

#[derive(Eq, PartialEq)]
pub enum Side { Left, Right }

#[derive(Component)]
pub struct ActiveWeapon(Side, Weapon);

#[derive(Component)]
pub struct JustFired(u16);

pub fn spawn_weapon(
    weapon: Weapons,
    side: Side,
    commands: &mut Commands,
    atlas: &Handle<TextureAtlas>
) {
    let mut weapon: Weapon = weapon.into();
    if side == Side::Right { weapon.tile.flip = !weapon.tile.flip };
    commands
        .spawn(ActiveWeapon(side, weapon.clone()))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(VisibilityBundle::default())
        .with_children(|spawn| { spawn.spawn(weapon.tile.sprite(0, 0, 0., atlas)); });
}

pub fn update_weapons(
    mut commands: Commands,
    ship: Query<&Transform, With<Ship>>,
    mut weapons: Query<(&ActiveWeapon, Option<&mut JustFired>, &mut Transform, Entity), Without<Ship>>,
) {
    let ship_pos = ship.single().translation;

    for (weapon, just_fired, mut pos, id) in weapons.iter_mut() {
        pos.translation.y = ship_pos.y + 16.;
        pos.translation.x = ship_pos.x + if weapon.0 == Side::Left { -8. } else { 32. };
        if let Some(mut just_fired) = just_fired {
            if just_fired.0 <= weapon.1.cooldown / 2 { pos.translation.x += if weapon.0 == Side::Left { 1. } else { -1. }; }
            just_fired.0 += 1;
            if just_fired.0 >= weapon.1.cooldown { commands.entity(id).remove::<JustFired>(); }
        }
    }
}

pub fn shoot(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    weapons: Query<(&ActiveWeapon, Option<&JustFired>, &Transform, Entity)>,
) {
    for (key_code, side) in [(KeyCode::Left, Side::Left), (KeyCode::Right, Side::Right)] {
        if keys.pressed(key_code) {
            for (weapon, just_fired, pos, id) in &weapons {
                if weapon.0 != side || just_fired.is_some() { continue }
                // TODO: Actually shoot
                commands.entity(id).insert(JustFired(0));
            }
        }
    }
}