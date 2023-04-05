use bevy::prelude::*;

use crate::util;
use crate::util::{Palette, size, z_pos};

pub fn spawn_chains(
    commands: &mut Commands,
    atlas: &Handle<TextureAtlas>
) {
    for y in 0..size::HEIGHT {
        for x in 0..2 {
            commands.spawn(
                util::sprite(
                    582 + if rand::random::<f32>() < 0.1 { 1 } else { 0 }, x + 15, y, z_pos::CHAINS,
                    Palette::TRANSPARENT, Palette::GRAY,
                    false, 0,
                    atlas.clone(),
                )
            );
        }
    }
}