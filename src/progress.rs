use bevy::prelude::Resource;

use crate::util;
use crate::weapons::Weapons;

#[derive(Resource)]
pub struct Progress {
    pub ship_speed: f32,
    pub damage_multiplier: f32,
    pub max_hp: u8,
    pub unlocked_weapons: Vec<Weapons>,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            ship_speed: util::ship::SPEED,
            damage_multiplier: 1.0,
            max_hp: 3,
            unlocked_weapons: vec![Weapons::Finger],
        }
    }
}