use bevy::prelude::*;
use strum_macros::EnumIter;

use crate::{GameState, MainBundle, util};
use crate::collision::{BodyType, Contact, SolidBody};
use crate::graphics::monsters::Monster;
use crate::graphics::ship::Ship;
use crate::graphics::tiles;
use crate::graphics::tiles::{Tile, Tiles};
use crate::loading::Textures;
use crate::util::{is_oob, Palette, Side, z_pos};
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
    pub name: char,
}

impl Weapon {
    pub const fn get_solid_body() -> SolidBody {
        SolidBody {
            body_type: BodyType::Ghost,
            width: tile_to_f32(1),
            height: tile_to_f32(1),
            bottom_right_anchor: false,
        }
    }
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
                Shot { piercing: true, speed: Vec2::new(0., 0.), ..Shot::default() },
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
            Weapons::Laser => Weapon {
                model: Weapons::Laser,
                shots: Shots::Laser,
                tile: Tiles::LaserCannon.to_tile().with_fg(Palette::Gravel),
                shot_tile: Tiles::LaserPreparing.to_tile().with_fg(Palette::LightRed),
                cooldown: 160,
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
                (update_weapons, shoot, update_shots, collide_shot, update_laser_shots,
                 switch_weapons)
                    .in_set(OnUpdate(GameState::Survival))
            );
    }
}

#[derive(Component)]
pub struct ActiveWeapon {
    pub side: Side,
    pub weapon: Weapon,
}

pub struct WeaponChanged(pub Side, pub Weapon);

pub enum LaserState {
    Loading,
    Firing
}

#[derive(Component)]
pub struct LaserShot {
    pub ref_entity: Entity,
    pub offset: Vec2,
    pub state: LaserState,
    pub frame: usize,
}

#[derive(Component)]
pub struct JustFired(u16);

#[derive(Component, Copy, Clone)]
pub struct Shot {
    piercing: bool,
    side: Side,
    speed: Vec2,
    damage: i16,
    dy: f32,
    dx: f32,
}

impl Default for Shot {
    fn default() -> Self {
        Self {
            piercing: false,
            side: Side::Left,
            speed: Default::default(),
            damage: 1,
            dy: 0.0,
            dx: 0.0,
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
        .spawn(ActiveWeapon { side, weapon })
        .insert(Weapon::get_solid_body())
        .insert(Transform::from_xyz(0., 0., z_pos::WEAPONS))
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
        let &ActiveWeapon { side, weapon: Weapon { cooldown, .. } } = weapon;
        pos.translation.x = ship_pos.x + if side == Side::Left { -2. } else { tile_to_f32(3) + 2. };
        pos.translation.y = ship_pos.y + tile_to_f32(2);
        if let Some(mut just_fired) = just_fired {
            if just_fired.0 <= cooldown / 2 { pos.translation.x += side.to_sign_f32(); }
            just_fired.0 += 1;
            if just_fired.0 >= cooldown {
                commands.entity(id).remove::<JustFired>();
            }
        }
    }
}

fn switch_weapons(
    mut commands: Commands,
    mut weapons: Query<(&mut ActiveWeapon, Entity), With<SolidBody>>,
    laser_shots: Query<Entity, With<LaserShot>>,
    keys: Res<Input<KeyCode>>,
    mut weapon_changed: EventWriter<WeaponChanged>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (mut active_weapon, e) in weapons.iter_mut() {
            let old_side = active_weapon.side;
            let new_side = old_side.flip();
            active_weapon.side = new_side;
            weapon_changed.send(WeaponChanged(new_side, active_weapon.weapon));
            commands.entity(e).insert(tiles::Flip);
        }

        for e in laser_shots.iter() {
            commands.entity(e).despawn_recursive()
        }
    }
}

fn shoot(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    weapons: Query<(&ActiveWeapon, Option<&JustFired>, &Transform, Entity), Without<Ship>>,
    ship: Query<Entity, With<Ship>>,
    textures: Res<Textures>,
) {
    for (key_code, side) in [(KeyCode::Left, Side::Left), (KeyCode::Right, Side::Right)] {
        if keys.pressed(key_code) {
            for (&ActiveWeapon { side: weapon_side, weapon }, just_fired, pos, id) in &weapons {
                if weapon_side != side || just_fired.is_some() { continue; }
                commands.entity(id).insert(JustFired(0));

                for &shot in weapon.model.get_shots().iter() {
                    spawn_shot(shot, &mut commands, &textures, side, weapon, pos, ship.get_single().ok());
                }
            }
        }
    }
}

fn spawn_shot(
    shot: Shot,
    commands: &mut Commands,
    textures: &Res<Textures>,
    side: Side,
    weapon: Weapon,
    pos: &Transform,
    ship: Option<Entity>,
) {
    let mut shot = shot.clone();
    let mut entity_commands = commands.spawn(shot.with_side(side));
    let entity_commands = entity_commands
        .insert(MainBundle::from_xyz(
            pos.translation.x - (tile_to_f32(1) + shot.dx) * side.to_sign_f32(),
            pos.translation.y + shot.dy,
            z_pos::SHOTS))
        .with_children(|spawn| {
            let mut tile = weapon.shot_tile;
            if side == Side::Right { tile = tile.flip(); }
            spawn.spawn(tile.sprite(0, 0, 0., &textures.mrmotext));
        });

    if weapon.shots == Shots::Laser && ship.is_some() {
        entity_commands
            .insert(LaserShot {
                ref_entity: ship.unwrap(),
                offset: Vec2::new(if side == Side::Left { -tile_to_f32(1) - 1. } else { tile_to_f32(4) + 1. }, tile_to_f32(2)),
                state: LaserState::Loading,
                frame: 0,
            });
    } else {
        entity_commands
            .insert(SolidBody {
                body_type: BodyType::ShipShot,
                width: tile_to_f32(1),
                height: tile_to_f32(1),
                bottom_right_anchor: false,
            });
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
    mut commands: Commands,
    mut shots: Query<(&mut LaserShot, &Shot, &mut Transform, Entity)>,
    children_query: Query<&Children>,
    ref_entity: Query<(&Transform, Option<&Ship>), Without<LaserShot>>,
    textures: Res<Textures>,
) {
    for (mut laser, shot, mut pos, id) in shots.iter_mut() {
        if let Ok((ref_transform, ship)) = ref_entity.get(laser.ref_entity) {
            // Update laser position (relative to ref_entity)
            pos.translation.x = ref_transform.translation.x + laser.offset.x + shot.dx;
            pos.translation.y = ref_transform.translation.y + laser.offset.y + shot.dy;
            pos.translation.z = z_pos::SHOTS;

            // Update laser state
            laser.frame += 1;
            match laser.state {
                // Fire laser
                LaserState::Loading if laser.frame >= util::fight::LASER_LOADING => {
                    laser.frame = 0;
                    laser.state = LaserState::Firing;

                    // Delete children
                    for child in children_query.iter_descendants(id) {
                        commands.entity(child).despawn_recursive();
                    }

                    // Add body and children
                    commands
                        .entity(id)
                        .insert(SolidBody {
                            body_type: if ship.is_some() { BodyType::ShipShot } else { BodyType::EnemyShot },
                            width: tile_to_f32(util::size::WIDTH),
                            height: tile_to_f32(1),
                            bottom_right_anchor: shot.side == Side::Left,
                        })
                        .with_children(|builder| {
                            let tile = Tiles::Laser.to_tile().with_fg(Palette::LightRed);
                            for x in 0..util::size::WIDTH {
                                let mut bundle = tile.sprite(0, 0, 0., &textures.mrmotext);
                                bundle.transform.translation.x = tile_to_f32(x) * if shot.side == Side::Left { -1. } else { 1. };
                                builder.spawn(bundle);
                            }
                        });
                }

                // Destroy laser
                LaserState::Firing if laser.frame >= util::fight::LASER_FIRING => {
                    commands.entity(id).despawn_recursive();
                }
                _ => ()
            }
        } else {
            commands.entity(id).despawn_recursive();
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
                        monster.lives -= shot.damage;
                    }
                }
            }
            (BodyType::ShipShot, BodyType::Enemy) => {
                if let Ok(shot) = shot_info.get(*id1) {
                    if let Ok(mut monster) = monsters.get_mut(*id2) {
                        monster.lives -= shot.damage;
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