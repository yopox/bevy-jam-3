use bevy::asset::Handle;
use bevy::prelude::{BuildChildren, Commands, Entity};
use bevy::sprite::TextureAtlas;
use strum_macros::EnumIter;

use crate::{collision, MainBundle, util};
use crate::collision::{BodyType, Hitbox, SolidBody};
use crate::graphics::sprites;
use crate::graphics::sprites::TILE;
use crate::util::{Palette, z_pos};
use crate::util::size::tile_to_f32;

#[derive(Debug, EnumIter)]
pub enum Monsters {
    CashKnight,
    MagicCandle,
    Necromancer,
    StarFly,
    SpaceCrab,
    SpaceShrimp,
    SuperEye,
}

impl Monsters {
    pub fn sprite(&self) -> &[TILE] {
        match self {
            Monsters::CashKnight => &sprites::CASH_KNIGHT,
            Monsters::MagicCandle => &sprites::MAGIC_CANDLE,
            Monsters::Necromancer => &sprites::NECROMANCER,
            Monsters::StarFly => &sprites::STAR_FLY,
            Monsters::SpaceCrab => &sprites::SPACE_CRAB,
            Monsters::SpaceShrimp => &sprites::SPACE_SHRIMP,
            Monsters::SuperEye => &sprites::SUPER_EYE,
        }
    }

    fn palette(&self) -> Vec<Palette> {
        match self {
            Monsters::CashKnight => vec![Palette::Transparent, Palette::Black, Palette::Gold],
            Monsters::MagicCandle => vec![Palette::Transparent, Palette::Black, Palette::LightGold, Palette::Red],
            Monsters::Necromancer => vec![Palette::Transparent, Palette::Black, Palette::LightGold],
            Monsters::StarFly => vec![Palette::Transparent, Palette::Black, Palette::Black],
            Monsters::SpaceCrab => vec![Palette::Transparent, Palette::Black, Palette::Lava],
            Monsters::SpaceShrimp => vec![Palette::Transparent, Palette::Black, Palette::Blue],
            Monsters::SuperEye => vec![Palette::Transparent, Palette::Black, Palette::Blue],
        }
    }
}

pub enum Families {
    Bats,
    Pharaoh,
    Color(Palette),
}

impl Families {
    fn color(&self) -> Palette {
        match self {
            Families::Bats => Palette::Blue,
            Families::Pharaoh => Palette::Gold,
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

    let body_size = collision::body_size(monster.sprite());
    commands
        .spawn(MainBundle::from_xyz(tile_to_f32(x), tile_to_f32(y), z_pos::ENEMIES))
        .insert(SolidBody {
            body_type: BodyType::Enemy,
            width: body_size.x,
            height: body_size.y,
        })
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
                if let Some(hitbox) = Hitbox::for_tile(i, bg == 0) {
                    commands.insert(hitbox);
                }
            }
        })
        .id()
}