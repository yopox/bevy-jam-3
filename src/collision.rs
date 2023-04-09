use bevy::app::{App, Plugin};
use bevy::hierarchy::HierarchyQueryExt;
use bevy::math::{vec2, vec3};
use bevy::prelude::{Children, Commands, Component, DetectChangesMut, Entity, EventReader, EventWriter, Query, Transform, Visibility, Without};
use bevy::sprite::collide_aabb;
use bevy::utils::default;
use strum::IntoEnumIterator;

use crate::graphics::monsters::Monsters;
use crate::graphics::sprites;
use crate::graphics::sprites::TILE;
use crate::util::{fight, Palette, size};
use crate::weapons::{Weapon, Weapons};

/// Handles collisions.
///
/// In order to get a collision we need:
/// - [SolidBody] on the parent with its size (translation + size / 2. = center)
/// - [Hitbox] on the children entities
/// A [Contact] event will be sent after the collision.
/// To pause collisions momentarily, add an [Invincible] component with the desired cooldown.
pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Contact>()
            .add_systems((collide, update_invincible, add_invincible));
    }
}

/// Takes entity into account for collision detection. Entity children should have a [Hitbox].
/// [body_type] is used to perform collision detection against the right bodies.
/// [width] and [height] describe a rectangle containing all the children [Hitbox]-es.
#[derive(Component)]
pub struct SolidBody {
    pub body_type: BodyType,
    pub width: f32,
    pub height: f32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum BodyType {
    Enemy,
    EnemyShot,
    Ship,
    ShipShot,
}

impl BodyType {
    fn can_collide(&self, other: &BodyType) -> bool {
        match (self, other) {
            (BodyType::Enemy, BodyType::ShipShot) | (BodyType::ShipShot, BodyType::Enemy) => true,
            (BodyType::Ship, BodyType::EnemyShot) | (BodyType::EnemyShot, BodyType::Ship) => true,
            _ => false
        }
    }
}

pub struct Contact(pub (BodyType, Entity), pub (BodyType, Entity));

/// Excludes the entity from collision detection.
#[derive(Component)]
pub struct Invincible(usize);

#[derive(Component, Default, Clone, Copy)]
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
            // Dash: full width, 1px height, 4px dy
            (877, _) => Some(Hitbox { width: 8.0, height: 1.0, dy: 4.0, ..default() }),
            // Empty tile, transparent: no hitbox
            (0, true) => None,
            // Enemy details, no hitbox
            (619, _) | (268, _) | (487, _) | (777, _) | (967, _)
            | (463, _) | (397, _) | (643, _) => None,
            // Ship border, no hitbox
            (56, _) | (59, _) | (231, _) => None,
            // Default case: whole box
            _ => Some(Hitbox { width: 8.0, height: 8.0, ..default() }),
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

pub fn body_size(sprite: &[TILE]) -> bevy::math::Vec2 {
    let x = *sprite.iter().map(|(x, _, _, _, _, _, _)| x).max().unwrap_or(&0);
    let y = *sprite.iter().map(|(_, y, _, _, _, _, _)| y).max().unwrap_or(&0);
    return vec2(size::tile_to_f32(x + 1), size::tile_to_f32(y + 1));
}

pub fn add_invincible(
    mut commands: Commands,
    mut contact: EventReader<Contact>,
    invincible: Query<Option<&Invincible>>,
) {
    for Contact((b1, e1), (b2, e2)) in contact.iter() {
        for (b, e) in [(b1, e1), (b2, e2)] {
            if *b == BodyType::Ship || *b == BodyType::Enemy {
                // TODO: Small freeze for enemies, global freeze if the ship is hit, long global freeze if ship is destroyed
                if let Ok(Some(_)) = invincible.get(*e) { continue }
                commands.entity(*e).insert(Invincible(fight::ENEMY_COOLDOWN));
                // info!("Hit!");
            }
        }
    }
}

pub fn update_invincible(
    mut commands: Commands,
    mut invincible: Query<(&mut Invincible, &mut Visibility, Entity)>,
) {
    for (mut inv, mut visibility, id) in invincible.iter_mut() {
        if inv.0 == 0 { commands.entity(id).remove::<Invincible>(); }
        else {
            inv.0 -= 1;
            visibility.set_if_neq(if (inv.0 / 20) % 2 == 0 {Visibility::Inherited}  else {Visibility::Hidden});
        }
    }
}

pub fn collide(
    colliders: Query<(&SolidBody, &Transform, Entity), Without<Invincible>>,
    children_query: Query<&Children>,
    hitboxes: Query<(&Hitbox, &Transform), Without<SolidBody>>,
    mut contact: EventWriter<Contact>,
) {
    let bodies = &colliders.iter().collect::<Vec<(&SolidBody, &Transform, Entity)>>();
    for (i, &(body1, pos1, id1)) in bodies.iter().enumerate() {
        'for_body: for &(body2, pos2, id2) in bodies.iter().skip(i) {
            if !body1.body_type.can_collide(&body2.body_type) { continue }

            // Collide outer bounds first to avoid complex computations
            // collide 1/3 args must be the center of the rectangle, 2/4 args are the rectangle size
            if collide_aabb::collide(
                vec3(pos1.translation.x + body1.width / 2., pos1.translation.y + body1.height / 2., 0.),
                vec2(body1.width, body1.height),
                vec3(pos2.translation.x + body2.width / 2., pos2.translation.y + body2.height / 2., 0.),
                vec2(body2.width, body2.height)
            ).is_none() { continue }

            // Collide entity 1 children with entity 2 children
            for child1 in children_query.iter_descendants(id1) {
                let Ok((hitbox1, cpos1)) = hitboxes.get(child1) else { continue };
                for child2 in children_query.iter_descendants(id2) {
                    let Ok((hitbox2, cpos2)) = hitboxes.get(child2) else { continue };

                    if collide_aabb::collide(
                        vec3(pos1.translation.x + cpos1.translation.x + hitbox1.dx + hitbox1.width / 2.,
                             pos1.translation.y + cpos1.translation.y + hitbox1.dy + hitbox1.height / 2., 0.),
                        vec2(hitbox1.width, hitbox1.height),
                        vec3(pos2.translation.x + cpos2.translation.x + hitbox2.dx + hitbox2.width / 2.,
                             pos2.translation.y + cpos2.translation.y + hitbox2.dy + hitbox2.height / 2., 0.),
                        vec2(hitbox2.width, hitbox2.height)
                    ).is_some() {
                        contact.send(Contact((body1.body_type, id1), (body2.body_type, id2)));
                        break 'for_body;
                    }
                }
            }
        }
    }
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