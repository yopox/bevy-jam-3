use bevy::asset::Handle;
use bevy::prelude::{BuildChildren, Commands, Entity};
use bevy::sprite::TextureAtlas;

use crate::{MainBundle, util};
use crate::graphics::sprites;
use crate::graphics::sprites::TILE;
use crate::util::{Palette, z_pos};
use crate::util::size::tile_to_f32;

pub enum Monsters {
    StarFly,
}

impl Monsters {
    fn sprite(&self) -> &[TILE] {
        match self {
            Monsters::StarFly => &sprites::STAR_FLY,
        }
    }

    fn palette(&self) -> Vec<Palette> {
        match self {
            Monsters::StarFly => vec![Palette::Transparent, Palette::Black, Palette::DarkBlue],
        }
    }
}

pub enum Families {
    Bats,
    Color(Palette),
}

impl Families {
    fn color(&self) -> Palette {
        match self {
            Families::Bats => Palette::Gray,
            Families::Color(p) => *p,
        }
    }
}

pub fn spawn_monster(
    commands: &mut Commands,
    atlas: &Handle<TextureAtlas>,
    monster: Monsters,
    family: Families,
    x: usize,
    y: usize,
) -> Entity {
    let mut palette = monster.palette();
    palette[1] = family.color();

    commands
        .spawn(MainBundle::from_xyz(tile_to_f32(x), tile_to_f32(y), z_pos::ENEMIES))
        .with_children(|builder| {
            for &(x, y, i, bg, fg, flip, rotation) in monster.sprite() {
                let mut commands = builder.spawn(
                    util::sprite(
                        i, x, y, 0.,
                        palette[bg], palette[fg],
                        flip, rotation,
                        atlas.clone(),
                    )
                );
            }
        })
        .id()
}