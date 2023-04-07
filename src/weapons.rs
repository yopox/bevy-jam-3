use bevy::prelude::*;

use crate::GameState;
use crate::graphics::ship::Ship;
use crate::graphics::tiles::{Tile, Tiles};
use crate::loading::Textures;
use crate::util::{is_oob, Palette, z_pos};

/// Kind of shots
#[derive(Copy, Clone)]
pub enum Shots {
    /// Straight line, 1 tile
    Simple,
    /// 2 shots, diagonal (45 deg)
    Double,
    /// Fill line (until obstacle) + follow player
    Laser,
}

/// Weapon description (left orientation)
#[derive(Copy, Clone)]
pub struct Weapon {
    pub shots: Shots,
    pub tile: Tile,
    pub shot_tile: Tile,
    pub cooldown: u16,
    pub name: char,
}

pub enum Weapons {
    Finger,
}

impl Into<Weapon> for Weapons {
    fn into(self) -> Weapon {
        match self {
            Weapons::Finger => Weapon {
                shots: Shots::Simple,
                tile: Tiles::LeftHand.to_tile().with_fg(Palette::DarkGray),
                shot_tile: Tiles::Dash.to_tile().with_fg(Palette::DarkGray),
                cooldown: 40,
                name: 'f',
            }
        }
    }
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<WeaponChanged>()
            .add_systems(
                (update_weapons, shoot, update_shots)
                    .in_set(OnUpdate(GameState::Survival))
            );
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Side { Left, Right }

impl Side {
    pub fn factor(&self) -> f32 {
        match self {
            Side::Left => 1.,
            Side::Right => -1.,
        }
    }
}

#[derive(Component)]
pub struct ActiveWeapon(pub Side, pub Weapon);

pub struct WeaponChanged(pub Side, pub Weapon);

#[derive(Component)]
pub struct JustFired(u16);

#[derive(Component)]
pub struct Shot {
    weapon: Weapon,
    side: Side,
    speed: Vec2,
}

pub fn spawn_weapon(
    weapon: Weapons,
    side: Side,
    commands: &mut Commands,
    atlas: &Handle<TextureAtlas>,
    weapon_changed: &mut EventWriter<WeaponChanged>,
) {
    let mut weapon: Weapon = weapon.into();
    if side == Side::Right { weapon.tile.flip = !weapon.tile.flip };
    commands
        .spawn(ActiveWeapon(side, weapon.clone()))
        .insert(Transform::default())
        .insert(GlobalTransform::default())
        .insert(VisibilityBundle::default())
        .with_children(|spawn| { spawn.spawn(weapon.tile.sprite(0, 0, 0., atlas)); });
    weapon_changed.send(WeaponChanged(side, weapon));
}

fn update_weapons(
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

fn shoot(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    weapons: Query<(&ActiveWeapon, Option<&JustFired>, &Transform, Entity)>,
    textures: Res<Textures>,
) {
    for (key_code, side) in [(KeyCode::Left, Side::Left), (KeyCode::Right, Side::Right)] {
        if keys.pressed(key_code) {
            for (weapon, just_fired, pos, id) in &weapons {
                if weapon.0 != side || just_fired.is_some() { continue; }
                commands.entity(id).insert(JustFired(0));

                commands
                    .spawn(Shot {
                        weapon: weapon.1,
                        side,
                        speed: Vec2::new(-1., 0.),
                    })
                    .insert(Transform::from_xyz(
                        if side == Side::Left { pos.translation.x - 8. } else { pos.translation.x + 8. },
                        pos.translation.y,
                        z_pos::SHOTS))
                    .insert(GlobalTransform::default())
                    .insert(VisibilityBundle::default())
                    .with_children(|spawn| { spawn.spawn(weapon.1.shot_tile.sprite(0, 0, 0., &textures.mrmotext)); });
            }
        }
    }
}

fn update_shots(
    mut commands: Commands,
    mut shots: Query<(&Shot, &mut Transform, Entity)>,
) {
    for (shot, mut transform, id) in shots.iter_mut() {
        transform.translation.x += shot.speed.x * shot.side.factor();
        transform.translation.y += shot.speed.y * shot.side.factor();

        if is_oob(&transform) { commands.entity(id).despawn_recursive(); }
    }
}