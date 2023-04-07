use bevy::prelude::*;

use crate::util;
use crate::util::{Palette, size, z_pos};

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

pub fn spawn_ship(
    commands: &mut Commands,
    atlas: &Handle<TextureAtlas>,
) {
    let colors = [
        Palette::TRANSPARENT,
        Palette::DARK_BLUE,
        Palette::BLACK,
    ];
    commands
        .spawn(Ship { y: 0 })
        .insert(Transform {
            translation: Vec3::new(
                size::WIDTH as f32 * size::TILE_SIZE / 2. - 16.,
                SHIP_INIT_Y,
                z_pos::MACHINE,
            ),
            ..default()
        })
        .insert(GlobalTransform::default())
        .insert(VisibilityBundle::default())
        .with_children(|builder| {
            for (x, y, i, bg, fg, flip, rotation) in
            [
                (0, 4, 0, 0, 1, false, 0),
                (1, 4, 1010, 0, 2, false, 0),
                (2, 4, 1010, 0, 2, true, 0),
                (3, 4, 0, 0, 1, false, 0),
                (0, 3, 62, 1, 0, false, 2),
                (1, 3, 605, 0, 1, true, 0),
                (2, 3, 605, 0, 1, false, 0),
                (3, 3, 62, 1, 0, true, 2),
                (0, 2, 337, 0, 1, false, 0),
                (1, 2, 0, 1, 0, true, 2),
                (2, 2, 0, 1, 0, true, 2),
                (3, 2, 337, 0, 1, true, 0),
                (0, 1, 62, 1, 0, true, 0),
                (1, 1, 605, 0, 1, false, 2),
                (2, 1, 605, 0, 1, true, 2),
                (3, 1, 62, 1, 0, false, 0),
                (0, 0, 0, 0, 1, false, 0),
                (1, 0, 1010, 0, 2, true, 2),
                (2, 0, 1010, 0, 2, false, 2),
                (3, 0, 0, 0, 1, false, 0),
            ] {
                builder.spawn(
                    util::sprite(
                        i, x, y, 0.,
                        colors[bg], colors[fg],
                        flip, rotation,
                        atlas.clone(),
                    )
                );
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