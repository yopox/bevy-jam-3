use bevy::prelude::*;
use bevy_text_mode::TextModeTextureAtlasSprite;

use crate::{MainBundle, util};
use crate::graphics::sprites;
use crate::graphics::text::glyph_index;
use crate::util::{Palette, size, z_pos};
use crate::util::size::tile_to_f32;
use crate::weapons::{Side, WeaponChanged};

const SHIP_SPEED: i64 = 3;
const SHIP_INIT_Y: f32 = size::HEIGHT as f32 * size::TILE_SIZE / 2. - 16.;
const SHIP_MAX_Y: i64 = 19;
const SHIP_MIN_Y: i64 = -17;

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
    let colors = [
        Palette::Transparent,
        Palette::DarkBlue,
        Palette::Black,
        Palette::White,
    ];
    commands
        .spawn(Ship { y: 0 })
        .insert(MainBundle::from_xyz(
            tile_to_f32(size::WIDTH) / 2. - 16.,
            SHIP_INIT_Y,
            z_pos::MACHINE,
        ))
        .with_children(|builder| {
            for (x, y, i, bg, fg, flip, rotation) in sprites::SHIP {
                let mut commands = builder.spawn(
                    util::sprite(
                        i, x, y, 0.,
                        colors[bg], colors[fg],
                        flip, rotation,
                        atlas.clone(),
                    )
                );
                if (x == 1 || x == 2) && y == 2 { commands.insert(ShipChar(if x == 1 { Side::Left } else { Side::Right })); }
            }
        });
}

pub fn update_ship_y(
    mut event_reader: EventReader<ShipMoveEvent>,
    mut ship: Query<&mut Ship>,
) {
    if event_reader.is_empty() { return; }

    let moved: i64 = event_reader.iter().map(|ShipMoveEvent(i)| i).sum();

    if moved != 0 {
        let mut ship = ship.single_mut();
        let mut new_y = moved + ship.y;
        if new_y < SHIP_MIN_Y { new_y = SHIP_MIN_Y };
        if new_y > SHIP_MAX_Y { new_y = SHIP_MAX_Y };
        ship.y = new_y;
    }
}

pub fn update_ship_image(
    mut query: Query<(&mut Transform, &Ship), Changed<Ship>>
) {
    if let Ok((mut transform, ship)) = query.get_single_mut() {
        transform.translation.y = SHIP_INIT_Y + (SHIP_SPEED * ship.y) as f32;
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