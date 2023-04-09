use bevy::prelude::*;
use strum_macros::EnumIter;

use crate::{GameState, MainBundle};
use crate::collision::{BodyType, Contact, Hitbox, SolidBody};
use crate::graphics::ship::Ship;
use crate::graphics::tiles::{Tile, Tiles};
use crate::loading::Textures;
use crate::util::{is_oob, Palette, z_pos};
use crate::util::size::tile_to_f32;

/// Kind of shots
#[derive(Copy, Clone)]
pub enum Shots {
    /// Straight line, 1 tile
    Simple,
    /// Straight shot, pierces through enemies
    Piercing,
    /// 2 shots, diagonal (45 deg)
    Double,
    /// Fill line (until obstacle) + follow player
    Laser,
}

impl Shots {
    fn destroy_on_contact(&self) -> bool {
        match self {
            Shots::Simple => true,
            Shots::Piercing => false,
            Shots::Double => true,
            Shots::Laser => false,
        }
    }
}

/// Weapon description (left orientation)
#[derive(Copy, Clone)]
pub struct Weapon {
    pub model: Weapons,
    pub shots: Shots,
    pub tile: Tile,
    pub shot_tile: Tile,
    pub cooldown: u16,
    pub name: char,
}

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum Weapons {
    Finger,
    DoubleCannon,
}

impl Weapons {
    fn get_shots(&self) -> Vec<Shot> {
        match self {
            Weapons::Finger => vec![
                Shot { piercing: false, side: Side::Left, speed: Vec2::new(-1., 0.), dy: 0., },
            ],
            Weapons::DoubleCannon => vec![
                Shot { piercing: false, side: Side::Left, speed: Vec2::new(-0.5, 0.5), dy: 3., },
                Shot { piercing: false, side: Side::Left, speed: Vec2::new(-0.5, -0.5), dy: -3., },
            ],
        }
    }
}

impl Into<Weapon> for Weapons {
    fn into(self) -> Weapon {
        match self {
            Weapons::Finger => Weapon {
                model: Weapons::Finger,
                shots: Shots::Simple,
                tile: Tiles::LeftHand.to_tile().with_fg(Palette::Lava),
                shot_tile: Tiles::Dash.to_tile().with_fg(Palette::Red),
                cooldown: 40,
                name: 'f',
            },
            Weapons::DoubleCannon => Weapon {
                model: Weapons::DoubleCannon,
                shots: Shots::Double,
                tile: Tiles::DoubleCannon.to_tile().with_fg(Palette::Terracotta),
                shot_tile: Tiles::Dot.to_tile().with_fg(Palette::LightTerracotta),
                cooldown: 80,
                name: 'd',
            },
        }
    }
}

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<WeaponChanged>()
            .add_systems(
                (update_weapons, shoot, update_shots, collide_shot)
                    .in_set(OnUpdate(GameState::Survival))
            );
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Side { Left, Right }

#[derive(Component)]
pub struct ActiveWeapon(pub Side, pub Weapon);

pub struct WeaponChanged(pub Side, pub Weapon);

#[derive(Component)]
pub struct JustFired(u16);

#[derive(Component, Copy, Clone)]
pub struct Shot {
    piercing: bool,
    side: Side,
    speed: Vec2,
    dy: f32,
}

impl Shot {
    fn with_side(&mut self, side: Side) -> Self {
        Self {
            piercing: self.piercing, dy: self.dy,
            side, speed: if side == Side::Left { self.speed } else { Vec2::new(self.speed.x * -1., self.speed.y) },
        }
    }
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
        pos.translation.x = ship_pos.x + if weapon.0 == Side::Left { -tile_to_f32(1) } else { tile_to_f32(4) };
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

                for mut shot in weapon.1.model.get_shots().iter_mut() {
                    spawn_shot(shot.with_side(side), &mut commands, &textures, side, weapon, pos);
                }
            }
        }
    }
}

fn spawn_shot(shot: Shot, commands: &mut Commands, textures: &Res<Textures>, side: Side, weapon: &ActiveWeapon, pos: &Transform) {
    commands
        .spawn(shot.clone())
        .insert(MainBundle::from_xyz(
            if side == Side::Left { pos.translation.x - tile_to_f32(1) } else { pos.translation.x + tile_to_f32(1) },
            pos.translation.y + shot.dy,
            z_pos::SHOTS))
        .insert(SolidBody {
            body_type: BodyType::ShipShot,
            width: tile_to_f32(1),
            height: tile_to_f32(1),
        })
        .with_children(|spawn| {
            let mut tile = weapon.1.shot_tile;
            if side == Side::Right { tile = tile.flip(); }
            let mut commands = spawn
                .spawn(tile.sprite(0, 0, 0., &textures.mrmotext));
            if let Some(hitbox) = Hitbox::for_tile(tile.index, tile.bg == Palette::Transparent) {
                commands.insert(hitbox);
            }
        });
}

fn update_shots(
    mut commands: Commands,
    mut shots: Query<(&Shot, &mut Transform, Entity)>,
) {
    for (shot, mut transform, id) in shots.iter_mut() {
        transform.translation.x += shot.speed.x;
        transform.translation.y += shot.speed.y;

        if is_oob(&transform) { commands.entity(id).despawn_recursive(); }
    }
}

fn collide_shot(
    mut commands: Commands,
    mut contact: EventReader<Contact>,
    shot_info: Query<&Shot>,
) {
    for Contact((body1, id1), (body2, id2)) in contact.iter() {
        destroy_shot(&mut commands, &shot_info, body1, id1, body2);
        destroy_shot(&mut commands, &shot_info, body2, id2, body1);
    }
}

fn destroy_shot(commands: &mut Commands, shot_info: &Query<&Shot>, body1: &BodyType, id1: &Entity, body2: &BodyType) {
    if *body1 == BodyType::ShipShot && *body2 == BodyType::Enemy {
        let Ok(shot) = shot_info.get(*id1) else { return };
        if !shot.piercing {
            commands.entity(*id1).despawn_recursive();
        }
    }
}