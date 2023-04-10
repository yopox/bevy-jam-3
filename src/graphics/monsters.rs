use bevy::asset::Handle;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::sprite::TextureAtlas;
use rand::prelude::SliceRandom;
use strum_macros::EnumIter;

use crate::{collision, MainBundle, util};
use crate::collision::{BodyType, Invincible, SolidBody};
use crate::graphics::sprites;
use crate::graphics::sprites::{RTEMO_PALETTE, TILE};
use crate::rounds::{CurrentRound, MovementTypes};
use crate::util::{Palette, Side, z_pos};
use crate::util::size::{tile_to_f32, WIDTH};

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum Monsters {
    CashKnight,
    MagicCandle,
    MrCactus,
    Necromancer,
    StarFly,
    SpaceCrab,
    SpaceShrimp,
    SuperEye,
    Skulleton,
    Bat,
    Shroom,
    Fox,
    Blob,
}

impl Monsters {
    pub fn random_boss() -> Self {
        *[Monsters::SuperEye, Monsters::Skulleton, Monsters::Blob, Monsters::CashKnight].choose(&mut rand::thread_rng()).unwrap()
    }

    pub fn random_non_boss() -> Self {
        *[Monsters::MagicCandle, Monsters::MrCactus, Monsters::Necromancer, Monsters::StarFly, Monsters::SpaceCrab, Monsters::SpaceShrimp, Monsters::Bat, Monsters::Shroom, Monsters::Fox,].choose(&mut rand::thread_rng()).unwrap()
    }

    pub fn is_boss(&self) -> bool {
        match self {
            Monsters::SuperEye | Monsters::Skulleton | Monsters::Blob | Monsters::CashKnight => true,
            _ => false,
        }
    }

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
            Monsters::Skulleton => &sprites::SKULLETON,
            Monsters::Bat => &sprites::BAT,
            Monsters::Shroom => &sprites::SHROOM,
            Monsters::Fox => &sprites::FOX,
            Monsters::Blob => &sprites::BLOB,





        }
    }

    fn palette(&self) -> Vec<Palette> {
        match self {
            Monsters::CashKnight => vec![Palette::Transparent, Palette::Black, Palette::Gold],
            Monsters::MrCactus => sprites::RTEMO_PALETTE.iter().map(|p| *p).collect::<Vec<Palette>>(),
            Monsters::Necromancer => vec![Palette::Transparent, Palette::Black, Palette::LightGold],
            Monsters::StarFly => vec![Palette::Transparent, Palette::Black, Palette::Black],
            Monsters::SpaceCrab => vec![Palette::Transparent, Palette::Black, Palette::Lava],
            Monsters::SpaceShrimp => vec![Palette::Transparent, Palette::Black, Palette::Blue],
            Monsters::SuperEye => vec![Palette::Transparent, Palette::Black, Palette::Blue],
            Monsters::Skulleton | Monsters::Bat | Monsters::MagicCandle | Monsters::Shroom | Monsters::Fox | Monsters::Blob => RTEMO_PALETTE.iter().map(|p| *p).collect::<Vec<Palette>>()
        }
    }

    fn hp(&self) -> i16 {
        match self {
            Monsters::CashKnight => 10,
            Monsters::MagicCandle => 4,
            Monsters::MrCactus => 8,
            Monsters::Necromancer => 6,
            Monsters::StarFly => 2,
            Monsters::SpaceCrab => 3,
            Monsters::SpaceShrimp => 4,
            Monsters::SuperEye => 25,
            Monsters::Skulleton => 20,
            Monsters::Bat => 4,
            Monsters::Shroom => 3,
            Monsters::Fox => 30,
            Monsters::Blob => 5,
        }
    }
}

#[derive(Copy, Clone)]
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
    monster: Monster,
    family: Families,
    x: f32,
    y: f32,
) -> Entity {
    let mut palette = monster.kind.palette();
    palette[1] = family.color();

    let sprite = monster.kind.sprite();
    let body_size = collision::body_size(sprite);
    commands
        .spawn(MainBundle::from_xyz(x, y, z_pos::ENEMIES))
        .insert(SolidBody {
            body_type: BodyType::Enemy,
            width: body_size.x,
            height: body_size.y,
            bottom_right_anchor: false,
        })
        .insert(monster.clone())
        .insert(MonsterLastMoved::default())
        .with_children(|builder| {
            for &(x, y, i, bg, fg, flip, rotation) in sprite {
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

#[derive(Copy, Clone)]
pub enum MonsterPath {
    /// Stays at the same position
    Static,
    /// Move with speed indicated
    Linear(Vec2),
    /// Move with sin
    Sinusoid {
        speed_x: f32,
        frequency_y: f32,
        amplitude_y: usize,
    },
}

impl MonsterPath {
    pub fn is_linear(&self) -> bool { match self {
        MonsterPath::Linear(_) => true,
        _ => false,
    } }

    pub fn compute_move(&self, init_pos: Vec2, t: f32, side: Side) -> Vec2 {
        match *self {
            MonsterPath::Static => init_pos,
            MonsterPath::Linear(v) => init_pos + v * t * side.to_sign_f32(),
            MonsterPath::Sinusoid { speed_x, frequency_y, amplitude_y } => {
                let dx = speed_x * t * side.to_sign_f32();
                let dy = tile_to_f32(amplitude_y) * (t * frequency_y).sin();
                Vec2 { x: init_pos.x + dx, y: init_pos.y + dy }
            }
        }
    }
}

#[derive(Component, Copy, Clone)]
pub struct Monster {
    pub kind: Monsters,
    pub lives: i16,
    pub path: MonsterPath,
    pub init_pos: Vec2,
    pub side: Side,
}

impl Monster {
    pub fn new(kind: Monsters, path: MonsterPath, x: f32, y: f32, side: Side) -> Self {
        Self { kind, lives: kind.hp(), path, init_pos: vec2(x, y), side }
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
            commands.entity(id).despawn_recursive();

            if monster.kind.is_boss() {
                commands.insert_resource(CurrentRound::new());
            }
        }
    }
}

#[derive(Component, Default)]
pub struct MonsterLastMoved {
    ago: usize,
}

pub fn move_monsters(
    mut monsters: Query<(&mut Transform, &mut MonsterLastMoved, &mut Monster, Option<&Invincible>)>,
) {
    for (mut monster_pos, mut monster_last_moved, mut monster, invincible) in monsters.iter_mut() {
        if invincible.is_some() && invincible.unwrap().0 > util::fight::ENEMY_COOLDOWN - util::fight::MONSTERS_FREEZE { continue; }

        if monster.path.is_linear() &&
            ((monster.side == Side::Left && monster_pos.translation.x > tile_to_f32(4))
        ||  (monster.side == Side::Right && monster_pos.translation.x < tile_to_f32(WIDTH - 8))) {
            monster.path = MovementTypes::Boss.to_path(monster.side);
            monster.init_pos = vec2(monster_pos.translation.x, monster_pos.translation.y);
            monster_last_moved.ago = 0;
        }

        monster_pos.translation = monster.compute_translation(monster_last_moved.ago as f32);
        monster_last_moved.ago += 1;
    }
}
