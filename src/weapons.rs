use bevy::prelude::*;
use bevy::utils::tracing::subscriber::with_default;
use strum_macros::EnumIter;

use crate::{GameState, MainBundle};
use crate::collision::{BodyType, Contact, Hitbox, SolidBody};
use crate::graphics::ship::Ship;
use crate::graphics::tiles::{Tile, Tiles};
use crate::loading::Textures;
use crate::survival::Monster;
use crate::util::{is_oob, Palette, z_pos};
use crate::util::ship::LASER_LENGTH;
use crate::util::size::tile_to_f32;

/// Kind of shots
#[derive(Copy, Clone, PartialEq, Eq)]
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
    pub replacement: u16,
    pub name: char,
}

#[derive(Debug, EnumIter, Copy, Clone, PartialEq, Eq)]
pub enum Weapons {
    Finger,
    DoubleCannon,
    Laser,
}

impl Weapons {
    fn get_shots(&self) -> Vec<Shot> {
        match self {
            Weapons::Finger => vec![
                Shot { speed: Vec2::new(-1., 0.), ..Shot::default() },
            ],
            Weapons::DoubleCannon => vec![
                Shot { speed: Vec2::new(-0.5, 0.5), dy: 3., ..Shot::default() },
                Shot { speed: Vec2::new(-0.5, -0.5), dy: -3., ..Shot::default() },
            ],
            Weapons::Laser => vec![
                Shot { piercing: true, speed: Vec2::new(0., 0.), width: LASER_LENGTH, dx: tile_to_f32(1), ..Shot::default() },
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
                replacement: 20,
                name: 'f',
            },
            Weapons::DoubleCannon => Weapon {
                model: Weapons::DoubleCannon,
                shots: Shots::Double,
                tile: Tiles::DoubleCannon.to_tile().with_fg(Palette::Terracotta),
                shot_tile: Tiles::Dot.to_tile().with_fg(Palette::LightTerracotta),
                cooldown: 80,
                replacement: 40,
                name: 'd',
            },
            Weapons::Laser => Weapon {
                model: Weapons::Laser,
                shots: Shots::Laser,
                tile: Tiles::LeftHand.to_tile().with_fg(Palette::LightBlue),
                shot_tile: Tiles::Dash.to_tile().with_fg(Palette::Gold),
                cooldown: 120,
                replacement: 0,
                name: 'l',
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
                (update_weapons, shoot, update_shots, collide_shot, update_laser_shots)
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
pub struct LaserShot(Side);

#[derive(Component)]
pub struct JustFired(u16, Option<Entity>);

#[derive(Component, Copy, Clone)]
pub struct Shot {
    piercing: bool,
    side: Side,
    speed: Vec2,
    damages: i16,
    dy: f32,
    dx: f32,
    width: usize,
}

impl Default for Shot {
    fn default() -> Self {
        Self {
            piercing: false,
            side: Side::Left,
            speed: Default::default(),
            damages: 1,
            dy: 0.0,
            dx: 0.0,
            width: 1,
        }
    }
}

impl Shot {
    fn with_side(&mut self, side: Side) -> Self {
        if side == Side::Left { *self } else {
            let speed = Vec2::new(self.speed.x * -1., self.speed.y);
            Self { side, speed, ..*self }
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
        pos.translation.y = ship_pos.y + tile_to_f32(2);
        pos.translation.x = ship_pos.x + if weapon.0 == Side::Left { -2. } else { tile_to_f32(3) + 2. };
        if let Some(mut just_fired) = just_fired {
            if just_fired.0 <= weapon.1.replacement { pos.translation.x += if weapon.0 == Side::Left { 1. } else { -1. }; }
            just_fired.0 += 1;
            if just_fired.0 >= weapon.1.cooldown {
                commands.entity(id).remove::<JustFired>();
                if let Some(e) = just_fired.1 {
                    commands.entity(e).despawn_recursive()
                }
            }
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
            for (&ActiveWeapon(weapon_side, weapon), just_fired, pos, id) in &weapons {
                if weapon_side != side || just_fired.is_some() { continue; }
                let mut opt_shot_id: Option<Entity> = None;

                for &shot in weapon.model.get_shots().iter() {
                    let shot_id = spawn_shot(shot, &mut commands, &textures, side, weapon, pos);

                    if weapon.model == Weapons::Laser {
                        opt_shot_id = Some(shot_id);
                    }
                }

                commands.entity(id).insert(JustFired(0, opt_shot_id));
            }
        }
    }
}

fn spawn_shot(shot: Shot, commands: &mut Commands, textures: &Res<Textures>, side: Side, weapon: Weapon, pos: &Transform) -> Entity {
    let dx = if side == Side::Left { -tile_to_f32(1) - shot.dx } else { tile_to_f32(1) + shot.dx };

    let mut shot = shot.clone();
    let mut entity_commands = commands.spawn(shot.with_side(side));
    let entity_commands = entity_commands
        .insert(MainBundle::from_xyz(
            pos.translation.x + dx,
            pos.translation.y + shot.dy,
            z_pos::SHOTS))
        .insert(SolidBody {
            body_type: BodyType::ShipShot,
            width: tile_to_f32(shot.width),
            height: tile_to_f32(1),
        })
        .with_children(|spawn| {
            let mut tile = weapon.shot_tile;
            if side == Side::Right { tile = tile.flip(); }
            let hitbox = Hitbox::for_tile(tile.index, tile.bg == Palette::Transparent)
                .expect("Weapons shots should have hitbox.");
            for x in 0..shot.width {
                spawn.spawn(tile.sprite(x, 0, 0., &textures.mrmotext)).insert(hitbox);
            }
        });

    if weapon.shots == Shots::Laser {
        entity_commands.insert(LaserShot(side)).id()
    } else {
        entity_commands.id()
    }
}

fn update_shots(
    mut commands: Commands,
    mut shots: Query<(&Shot, &mut Transform, Entity), Without<LaserShot>>,
) {
    for (shot, mut transform, id) in shots.iter_mut() {
        transform.translation.x += shot.speed.x;
        transform.translation.y += shot.speed.y;

        if is_oob(&transform) { commands.entity(id).despawn_recursive(); }
    }
}

fn update_laser_shots(
    mut shots: Query<(&Shot, &mut Transform), (With<LaserShot>, Without<Ship>)>,
    ship: Query<&mut Transform, (With<Ship>, Without<Shot>)>,
) {
    if let Ok(ship_transform) = ship.get_single() {
        for (&shot, mut pos) in shots.iter_mut() {
            pos.translation.x = ship_transform.translation.x + shot.dx + if shot.side == Side::Left { -tile_to_f32(1) } else { tile_to_f32(4) };
            pos.translation.y = ship_transform.translation.y + shot.dy + tile_to_f32(2);
            pos.translation.z = z_pos::SHOTS;
        }
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

pub fn monster_looses_life(
    mut contact: EventReader<Contact>,
    mut monsters: Query<&mut Monster>,
    shot_info: Query<&Shot>,
) {
    for Contact((body1, id1), (body2, id2)) in contact.iter() {
        match (body1, body2) {
            (BodyType::Enemy, BodyType::ShipShot) => {
                if let Ok(shot) = shot_info.get(*id2) {
                    if let Ok(mut monster) = monsters.get_mut(*id1) {
                        monster.lives -= shot.damages;
                    }
                }
            }
            (BodyType::ShipShot, BodyType::Enemy) => {
                if let Ok(shot) = shot_info.get(*id1) {
                    if let Ok(mut monster) = monsters.get_mut(*id2) {
                        monster.lives -= shot.damages;
                    }
                }
            }
            (_, _) => {}
        }
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