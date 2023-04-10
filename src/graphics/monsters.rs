use bevy::asset::Handle;
use bevy::prelude::*;
use bevy::sprite::TextureAtlas;
use strum_macros::EnumIter;

use crate::{collision, MainBundle, util};
use crate::collision::{BodyType, Invincible, SolidBody};
use crate::graphics::sprites;
use crate::graphics::sprites::TILE;
use crate::util::{Palette, Side, z_pos};
use crate::util::size::{tile_to_f32, WIDTH};

#[derive(Debug, EnumIter)]
pub enum Monsters {
    CashKnight,
    MagicCandle,
    MrCactus,
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
            Monsters::MrCactus => &sprites::MR_CACTUS,
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
            Monsters::MrCactus => sprites::RTEMO_PALETTE.iter().map(|p| *p).collect::<Vec<Palette>>(),
            Monsters::Necromancer => vec![Palette::Transparent, Palette::Black, Palette::LightGold],
            Monsters::StarFly => vec![Palette::Transparent, Palette::Black, Palette::Black],
            Monsters::SpaceCrab => vec![Palette::Transparent, Palette::Black, Palette::Lava],
            Monsters::SpaceShrimp => vec![Palette::Transparent, Palette::Black, Palette::Blue],
            Monsters::SuperEye => vec![Palette::Transparent, Palette::Black, Palette::Blue],
        }
    }

    fn to_monster(&self, side: Side) -> Monster {
        match self {
            Monsters::MagicCandle => Monster::new(5, Vec2::new(1. / 29., 0.), side),
            Monsters::CashKnight => Monster::new(10, Vec2::new(1. / 31., 0.), side),
            Monsters::MrCactus => Monster::new(10, Vec2::new(1. / 53., 0.), side),
            Monsters::Necromancer => Monster::new(5, Vec2::new(1. / 9., 0.), side),
            Monsters::StarFly => Monster::new(20, Vec2::new(1. / 12., 0.), side),
            Monsters::SpaceCrab => Monster::new(30, Vec2::new(1. / 37., 0.), side),
            Monsters::SpaceShrimp => Monster::new(30, Vec2::new(1. / 41., 0.), side),
            Monsters::SuperEye => Monster::new(100, Vec2::new(1. / 100., 0.), side),
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
            bottom_right_anchor: false,
        })
        .insert(monster.to_monster(Side::of_x(x)))
        .insert(MonsterLastMoved::default())
        .with_children(|builder| {
            for &(x, y, i, bg, fg, flip, rotation) in monster.sprite() {
                builder.spawn(
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

#[derive(Component)]
pub struct Monster {
    pub lives: i16,
    pub speed: Vec2,
    pub side: Side,
}

impl Monster {
    pub fn new(lives: i16, speed: Vec2, side: Side) -> Self {
        Self { lives, speed, side }
    }
}

pub fn monster_dies(
    monsters: Query<(&Monster, &Invincible, Entity), Changed<Invincible>>,
    mut commands: Commands,
) {
    for (monster, invincible, id) in monsters.iter() {
        if monster.lives <= 0 && invincible.0 == 0 {
            // Should put an animation, freeze something or whatever
            commands.entity(id).despawn_recursive();
        }
    }
}

#[derive(Component, Default)]
pub struct MonsterLastMoved {
    ago: usize,
}

pub fn move_monsters(
    mut monsters: Query<(&mut Transform, &mut MonsterLastMoved, &Monster), Without<Invincible>>,
) {
    for (mut monster_pos, mut monster_last_moved, monster) in monsters.iter_mut() {
        if monster_last_moved.ago as f32 * monster.speed.x > tile_to_f32(1) {
            monster_last_moved.ago = 0;
            if monster_pos.translation.x < tile_to_f32(WIDTH / 2 - 2) || tile_to_f32(WIDTH / 2 - 1) < monster_pos.translation.x {
                monster_pos.translation.x += tile_to_f32(1) * monster.side.to_sign_f32();
            }
        } else {
            monster_last_moved.ago += 1;
        }
    }
}
