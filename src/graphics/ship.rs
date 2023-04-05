use bevy::prelude::*;
use crate::util;
use crate::util::{Palette, size, z_pos};

#[derive(Component)]
pub struct Ship;

pub fn spawn_ship(
    commands: &mut Commands,
    atlas: &Handle<TextureAtlas>,
) {
    let colors = [
        Palette::TRANSPARENT,
        Palette::DARK_BLUE,
        Palette::RED,
    ];
    commands
        .spawn(Ship)
        .insert(Transform {
            translation: Vec3::new(
                size::WIDTH as f32 * size::TILE_SIZE / 2. - 16.,
                size::HEIGHT as f32 * size::TILE_SIZE / 2. - 16.,
                z_pos::MACHINE,
            ),
            ..default()
        })
        .insert(GlobalTransform::default())
        .insert(VisibilityBundle::default())
        .with_children(|builder| {
            for (x, y, i, bg, fg, flip, rotation) in
            [
                (0, 4, 150, 0, 1, false, 0),
                (1, 4, 604, 0, 2, false, 0),
                (2, 4, 604, 0, 2, true, 0),
                (3, 4, 150, 0, 1, true, 0),
                (0, 3, 337, 0, 1, false, 0),
                (1, 3, 0, 1, 1, false, 0),
                (2, 3, 0, 1, 1, false, 0),
                (3, 3, 337, 0, 1, false, 2),
                (0, 2, 337, 0, 1, false, 0),
                (1, 2, 0, 1, 1, false, 0),
                (2, 2, 0, 1, 1, false, 0),
                (3, 2, 337, 0, 1, false, 2),
                (0, 1, 337, 0, 1, false, 0),
                (1, 1, 0, 1, 1, false, 0),
                (2, 1, 0, 1, 1, false, 0),
                (3, 1, 337, 0, 1, false, 2),
                (0, 0, 150, 0, 1, true, 2),
                (1, 0, 604, 0, 2, true, 2),
                (2, 0, 604, 0, 2, false, 2),
                (3, 0, 150, 0, 1, false, 2),
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