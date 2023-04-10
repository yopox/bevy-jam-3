use bevy::asset::Handle;
use bevy::prelude::*;
use bevy::prelude::system_adapter::new;
use bevy::sprite::TextureAtlas;
use rand::random;
use strum_macros::EnumIter;

use crate::{collision, MainBundle, util};
use crate::collision::{BodyType, Invincible, SolidBody};
use crate::graphics::sprites;
use crate::graphics::sprites::TILE;
use crate::util::{Palette, Side, z_pos};
use crate::util::choose::{BORDER_HEIGHT, BORDER_WIDTH};
use crate::util::size::{HEIGHT, tile_to_f32, WIDTH};

fn random_pos_on_side(side: Side) -> Vec2 {
    let x = match side {
        Side::Left => tile_to_f32(2),
        Side::Right => tile_to_f32(WIDTH - 2),
    };
    let y = random::<f32>() * tile_to_f32(HEIGHT - 2 * 3) + tile_to_f32(3);
    Vec2 { x, y }
}

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
            Monsters::MagicCandle => Monster::new_with_random_pos(5, MonsterPath::Linear(Vec2::new(1. / 29., 0.)), side),
            Monsters::CashKnight => Monster::new_with_random_pos(10, MonsterPath::Linear(Vec2::new(1. / 31., 0.)), side),
            Monsters::MrCactus => Monster::new_with_random_pos(10, MonsterPath::Linear(Vec2::new(1. / 53., 0.)), side),
            Monsters::Necromancer => Monster::new_with_random_pos(5, MonsterPath::Linear(Vec2::new(1. / 9., 0.)), side),
            Monsters::StarFly => Monster::new_with_random_pos(20, MonsterPath::Sinusoid { speed_x: 1. / 12., frequency_y: 1. / 12., amplitude_y: tile_to_f32(3) }, side),
            Monsters::SpaceCrab => Monster::new_with_random_pos(30, MonsterPath::Sinusoid { speed_x: 1. / 37., frequency_y: 1. / 37., amplitude_y: tile_to_f32(4) }, side),
            Monsters::SpaceShrimp => Monster::new_with_random_pos(30, MonsterPath::Linear(Vec2::new(1. / 41., 0.)), side),
            Monsters::SuperEye => Monster::new_with_random_pos(100, MonsterPath::Linear(Vec2::new(1. / 100., 0.)), side),
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

pub enum MonsterPath {
    /// Stays at the same position
    Static,
    /// Move with speed indicated
    Linear(Vec2),
    /// Move with sin
    Sinusoid {
        speed_x: f32,
        frequency_y: f32,
        amplitude_y: f32,
    },
}

impl MonsterPath {
    pub fn compute_move(&self, init_pos: Vec2, t: f32, side: Side) -> Vec2 {
        match *self {
            MonsterPath::Static => init_pos,
            MonsterPath::Linear(v) => init_pos + v * t * side.to_sign_f32(),
            MonsterPath::Sinusoid { speed_x, frequency_y, amplitude_y } => {
                let dx = speed_x * t * side.to_sign_f32();
                let dy = amplitude_y * (t * frequency_y).sin();
                Vec2 { x: init_pos.x + dx, y: init_pos.y + dy }
            }
        }
    }
}

#[derive(Component)]
pub struct Monster {
    pub lives: i16,
    pub path: MonsterPath,
    pub init_pos: Vec2,
    pub side: Side,
}

impl Monster {
    pub fn new(lives: i16, path: MonsterPath, init_pos: Vec2, side: Side) -> Self {
        Self { lives, path, init_pos, side }
    }

    pub fn new_with_random_pos(lives: i16, path: MonsterPath, side: Side) -> Self {
        Self::new(lives, path, random_pos_on_side(side), side)
    }

    pub fn compute_translation(&self, t: f32) -> Vec3 {
        let Vec2 { x, y } = self.path.compute_move(self.init_pos, t, self.side);
        Vec3 { x, y, z: z_pos::ENEMIES }
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
        if monster_pos.translation.x < tile_to_f32(WIDTH / 2 - 2) || tile_to_f32(WIDTH / 2 - 1) < monster_pos.translation.x {
            monster_pos.translation = monster.compute_translation(monster_last_moved.ago as f32);
            monster_last_moved.ago += 1;
        } else {
            // We should do something here, like loose life or whatever.
        }
    }
}
