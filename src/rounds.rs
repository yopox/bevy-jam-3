use std::collections::VecDeque;

use bevy::math::vec2;
use bevy::prelude::{Commands, Res, ResMut, Resource};
use rand::prelude::IteratorRandom;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::collision;
use crate::graphics::monsters::{Families, Monster, MonsterPath, Monsters, spawn_monster};
use crate::loading::Textures;
use crate::util::Side;
use crate::util::size::{tile_to_f32, WIDTH};

#[derive(EnumIter, Copy, Clone)]
pub enum MovementTypes {
    Straight,
    SineSmall,
    SineBig,
    Random,
}

impl MovementTypes {
    fn random() -> Self {
        MovementTypes::iter().choose(&mut rand::thread_rng()).unwrap()
    }

    fn to_path(self, side: Side) -> MonsterPath {
        match self {
            MovementTypes::Straight => MonsterPath::Linear(vec2(side.to_sign_f32(), 0.)),
            MovementTypes::SineSmall => MonsterPath::Sinusoid {
                speed_x: 0.1,
                frequency_y: 0.01,
                amplitude_y: 4,
            },
            MovementTypes::SineBig => MonsterPath::Sinusoid {
                speed_x: 0.2,
                frequency_y: 0.04,
                amplitude_y: 1,
            },
            MovementTypes::Random => MovementTypes::random().to_path(side),
        }
    }
}

pub type RoundEvent = (Time, Side, Y, Monsters, Families, MovementTypes);

pub struct Time(u64);
pub struct Y(usize);

#[derive(EnumIter)]
pub enum Rounds {
    Round1,
}

impl Rounds {
    pub(crate) fn random() -> Self {
        Rounds::iter().choose(&mut rand::thread_rng()).unwrap()
    }

    fn get_events(&self) -> Vec<RoundEvent> {
        match self {
            Rounds::Round1 => vec![
                (Time(60), Side::Right, Y(6), Monsters::MrCactus, Families::Pharaoh, MovementTypes::SineBig),
                (Time(150), Side::Left, Y(3), Monsters::MagicCandle, Families::Pharaoh, MovementTypes::SineSmall),
            ]
        }
    }
}

#[derive(Resource)]
pub struct CurrentRound {
    events: VecDeque<RoundEvent>,
    frame: u64,
}

impl From<Rounds> for CurrentRound {
    fn from(value: Rounds) -> Self {
        CurrentRound { events: VecDeque::from(value.get_events()), frame: 0 }
    }
}

pub fn update(
    mut commands: Commands,
    mut round: Option<ResMut<CurrentRound>>,
    textures: Res<Textures>,
) {
    let Some(mut round) = round else { return; };

    round.frame += 1;
    if round.events.is_empty() { return; }
    let &(Time(t), side, Y(y), monster, family, movement) = round.events.get(0).unwrap();

    if t <= round.frame {
        round.events.pop_front();

        let body_size = collision::body_size(monster.sprite());
        let x = if side == Side::Left { -body_size.x } else { tile_to_f32(WIDTH) };
        let y = tile_to_f32(y + 3);

        let monster = Monster::new(monster, movement.to_path(side), x, y, side);
        spawn_monster(&mut commands, &textures.mrmotext, monster, family, x, y);
    }
}