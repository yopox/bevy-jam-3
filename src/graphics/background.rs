use bevy::prelude::*;

use crate::util;
use crate::util::{Palette, size, z_pos};

pub fn spawn_rails(
    commands: &mut Commands,
    atlas: &Handle<TextureAtlas>
) {
    for y in 0..size::HEIGHT {
        for x in 0..2 {
            commands.spawn(
                util::sprite(
                    if rand::random::<f32>() < 0.1 { 299 } else { 331 }, x + 15, y, z_pos::RAILS,
                    Palette::Transparent, Palette::Gravel,
                    x == 1, 0,
                    atlas.clone(),
                )
            );
        }
    }
}