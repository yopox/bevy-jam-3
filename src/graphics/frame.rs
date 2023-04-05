use bevy::prelude::{Commands, Handle};
use bevy::sprite::TextureAtlas;

use crate::graphics::text::Text;
use crate::util;
use crate::util::{Palette, size, z_pos};

pub fn spawn_frame(commands: &mut Commands, atlas: &Handle<TextureAtlas>) {
    for y in 0..size::HEIGHT {
        for x in 0..size::WIDTH {
            if x < 2 || x + 3 > size::WIDTH || y < 3 || y + 2 > size::HEIGHT {
                commands.spawn(
                    util::sprite(
                        0, x, y, z_pos::FRAME,
                        Palette::BLACK, Palette::BLACK,
                        false, 0,
                        atlas.clone(),
                    )
                );
            }
        }
    }

    for (x, y, rotation) in [
        (size::WIDTH - 3, size::HEIGHT - 2, 0),
        (2, size::HEIGHT - 2, 3),
        (2, 3, 2),
        (size::WIDTH - 3, 3, 1),
    ] {
        commands.spawn(util::sprite(
            223, x, y, z_pos::FRAME, Palette::BLACK, Palette::TRANSPARENT,
            false, rotation, atlas.clone()
        ));
    }

    commands.spawn(Text::from_str("score: 000000", 3, 1, z_pos::GUI));
}