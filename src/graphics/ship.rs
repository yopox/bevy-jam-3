use bevy::prelude::*;
use bevy_text_mode::TextModeTextureAtlasSprite;

use crate::{MainBundle, util};
use crate::graphics::sprites;
use crate::graphics::text::glyph_index;
use crate::util::{Palette, ship, size, z_pos};
use crate::util::size::tile_to_f32;
use crate::weapons::{Side, WeaponChanged};

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
            ship::INIT_Y,
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