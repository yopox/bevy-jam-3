use bevy::prelude::*;
use bevy_text_mode::TextModeTextureAtlasSprite;

use crate::{collision, MainBundle, util};
use crate::collision::{BodyType, Contact, Invincible, SolidBody};
use crate::graphics::animation::NoAnimation;
use crate::graphics::monsters::Monster;
use crate::graphics::sprites;
use crate::graphics::text::glyph_index;
use crate::survival::{Life, SurvivalUI};
use crate::util::{ship, Side, size, z_pos};
use crate::util::size::tile_to_f32;
use crate::weapons::WeaponChanged;

#[derive(Component)]
pub struct Ship {
    y: i64,
}

#[derive(Default)]
pub struct ShipMoveEvent(pub i64);

#[derive(Component)]
pub struct ShipChar(pub Side);

pub fn spawn_ship(
    commands: &mut Commands,
    atlas: &Handle<TextureAtlas>,
) {
    let body_size = collision::body_size(&sprites::SHIP);
    let colors = sprites::RTEMO_PALETTE;
    commands
        .spawn(Ship { y: 0 })
        .insert(SurvivalUI)
        .insert(MainBundle::from_xyz(
            tile_to_f32(size::WIDTH) / 2. - tile_to_f32(2),
            ship::INIT_Y,
            z_pos::MACHINE,
        ))
        .insert(SolidBody {
            body_type: BodyType::Ship,
            width: body_size.x,
            height: body_size.y,
            bottom_right_anchor: false,
        })
        .with_children(|builder| {
            for (x, y, i, bg, fg, flip, rotation) in sprites::SHIP {
                let mut commands = builder
                    .spawn(util::sprite(
                        i, x, y, 0.,
                        colors[bg], colors[fg],
                        flip, rotation,
                        atlas.clone(),
                    ));
                commands.insert(NoAnimation);
                if (x == 1 || x == 2) && y == 2 { commands.insert(ShipChar(if x == 1 { Side::Left } else { Side::Right })); }
            }
        });
}

pub fn update_ship_y(
    keys: Res<Input<KeyCode>>,
    mut ship: Query<&mut Ship>,
) {
    let moved = {
        let mut moved = 0;
        if keys.pressed(KeyCode::Up) { moved += 1 }
        if keys.pressed(KeyCode::Down) { moved -= 1 }
        moved
    };

    if moved != 0 {
        let mut ship = ship.single_mut();
        let mut new_y = moved + ship.y;
        if new_y < ship::MIN_Y { new_y = ship::MIN_Y };
        if new_y > ship::MAX_Y { new_y = ship::MAX_Y };
        ship.y = new_y;
    }
}

pub fn update_ship_image(
    mut query: Query<(&mut Transform, &Ship), Changed<Ship>>
) {
    if let Ok((mut transform, ship)) = query.get_single_mut() {
        transform.translation.y = ship::INIT_Y + ship::SPEED * ship.y as f32;
    }
}

pub fn update_ship_name(
    mut weapon_changed: EventReader<WeaponChanged>,
    mut ship_char: Query<(&mut TextModeTextureAtlasSprite, &ShipChar)>,
) {
    for WeaponChanged(side, weapon) in weapon_changed.iter() {
        ship_char.for_each_mut(|(mut sprite, ship_char)| {
            if ship_char.0 == *side { sprite.index = glyph_index(weapon.name).expect("Couldn't find weapon name glyph index.") }
        })
    }
}

pub fn monsters_kill(
    mut life: Query<&mut Life>,
    mut contacts: EventReader<Contact>,
    mut monsters: Query<&mut Monster, Without<Invincible>>,
) {
    for Contact((body1, id1), (body2, id2)) in contacts.iter() {
        match ((body1, id1), (body2, id2)) {
            ((BodyType::Enemy, id_enemy), (BodyType::Ship, id_ship)) |
            ((BodyType::Ship, id_ship), (BodyType::Enemy, id_enemy))
            => {
                if let Ok(mut life) = life.get_single_mut() {
                    life.0 -= 1;
                }
                if let Ok(mut monster) = monsters.get_mut(*id_enemy) {
                    monster.lives = 0;
                }
            }
            _ => {}
        }
    }
}
