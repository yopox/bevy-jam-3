use bevy::math::vec2;
use bevy::prelude::{Commands, Res, ResMut, Resource};
use rand::prelude::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{collision, util};
use crate::graphics::monsters::{Families, Monster, MonsterPath, Monsters, spawn_monster};
use crate::loading::Textures;
use crate::util::{Palette, Side};
use crate::util::size::{tile_to_f32, WIDTH};

#[derive(EnumIter, Copy, Clone)]
pub enum MovementTypes {
    Straight,
    SineSmall,
    SineBig,
    Boss,
    Random,
}

impl MovementTypes {
    fn random() -> Self {
        *[MovementTypes::SineSmall, MovementTypes::SineBig].iter().choose(&mut rand::thread_rng()).unwrap()
    }

    pub fn to_path(self, side: Side) -> MonsterPath {
        match self {
            MovementTypes::Straight => MonsterPath::Linear(vec2(side.to_sign_f32() / 3., 0.)),
            MovementTypes::SineSmall => MonsterPath::Sinusoid {
                speed_x: 0.1,
                frequency_y: 0.01,
                amplitude_y: 1,
            },
            MovementTypes::SineBig => MonsterPath::Sinusoid {
                speed_x: 0.2,
                frequency_y: 0.04,
                amplitude_y: 2,
            },
            MovementTypes::Boss => MonsterPath::Sinusoid {
                speed_x: 0.0,
                frequency_y: 0.03,
                amplitude_y: 3,
            },
            MovementTypes::Random => MovementTypes::random().to_path(side),
        }
    }
}

pub type RoundEvent = (Time, Side, Y, Monsters, Families, MovementTypes);

pub struct Time(u64);
pub struct Y(usize);

#[derive(Resource)]
pub struct CurrentRound {
    frame: u64,
}

impl CurrentRound {
    pub fn new() -> Self {
        CurrentRound { frame: 0 }
    }
}

pub fn update(
    mut commands: Commands,
    mut round: Option<ResMut<CurrentRound>>,
    textures: Res<Textures>,
) {
    let Some(mut round) = round else { return; };

    if round.frame % util::fight::MONSTER_SPAWN_INTERVAL == 0 {
        if round.frame >= util::fight::MONSTER_SPAWN_INTERVAL * util::fight::BOSS_AFTER {
            // Spawn boss
            let (side, monster, family, movement) = (
                if rand::random() { Side::Left } else { Side::Right },
                Monsters::random_boss(),
                Families::Color(Palette::random()),
                MovementTypes::Straight,
            );

            let body_size = collision::body_size(monster.sprite());
            let x = if side == Side::Left { -body_size.x } else { tile_to_f32(WIDTH) };
            let y = tile_to_f32(7);

            let monster = Monster::new(monster, movement.to_path(side), x, y, side);
            spawn_monster(&mut commands, &textures.mrmotext, monster, family, x, y);

            commands.remove_resource::<CurrentRound>();
        } else {
            // Spawn normal enemy
            let (side, Y(y), monster, family, movement) = (
                if rand::random() { Side::Left } else { Side::Right },
                Y(rand::random::<usize>() % 6 + 3),
                Monsters::random_non_boss(),
                Families::Color(Palette::random()),
                MovementTypes::Random,
            );

            let body_size = collision::body_size(monster.sprite());
            let x = if side == Side::Left { -body_size.x } else { tile_to_f32(WIDTH) };
            let y = tile_to_f32(y + 3);

            let monster = Monster::new(monster, movement.to_path(side), x, y, side);
            spawn_monster(&mut commands, &textures.mrmotext, monster, family, x, y);
        }
    }

    round.frame += 1;
}