use std::cmp::{max, min};

use bevy::prelude::*;

use crate::collision::Invincible;
use crate::GameState;
use crate::graphics::frame::spawn_frame;
use crate::graphics::monsters::{Families, Monsters, spawn_monster};
use crate::graphics::ship::{ShipMoveEvent, spawn_ship, update_ship_image, update_ship_name, update_ship_y};
use crate::graphics::text;
use crate::graphics::text::{color_text, text};
use crate::loading::Textures;
use crate::util::{Palette, Side, z_pos};
use crate::util::size::{tile_to_f32, WIDTH};
use crate::weapons::{monster_looses_life, spawn_weapon, WeaponChanged, Weapons};

pub struct SurvivalPlugin;

impl Plugin for SurvivalPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShipMoveEvent>()
            .add_system(setup.in_schedule(OnEnter(GameState::Survival)))
            .add_systems(
                (update_score, increase_score, update_life, update_ship_image, update_ship_y,
                 update_ship_name, monster_looses_life, monster_dies, move_monsters)
                    .in_set(OnUpdate(GameState::Survival))
            )
            .add_system(cleanup.in_schedule(OnExit(GameState::Survival)));
    }
}

#[derive(Component)]
struct SurvivalUI;

#[derive(Component)]
struct Score(i64);

#[derive(Component)]
struct Life(i8);

const LIFE_TEXTS: [&str; 6] = ["°°°°°", "•°°°°", "••°°°", "•••°°", "••••°", "•••••"];


fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
    mut weapon_changed: EventWriter<WeaponChanged>,
) {
    spawn_frame(&mut commands, &textures.mrmotext);
    spawn_ship(&mut commands, &textures.mrmotext);
    spawn_weapon(Weapons::Laser, Side::Left, &mut commands, &textures.mrmotext, &mut weapon_changed);
    spawn_monster(&mut commands, &textures.mrmotext, Monsters::StarFly, Families::Bats, 23, 14);
    spawn_monster(&mut commands, &textures.mrmotext, Monsters::SpaceCrab, Families::Color(Palette::Red), 20, 11);
    spawn_monster(&mut commands, &textures.mrmotext, Monsters::CashKnight, Families::Bats, 8, 13);
    spawn_monster(&mut commands, &textures.mrmotext, Monsters::SpaceShrimp, Families::Color(Palette::LightBlue), 22, 7);
    spawn_monster(&mut commands, &textures.mrmotext, Monsters::SuperEye, Families::Pharaoh, 5, 5);
    spawn_monster(&mut commands, &textures.mrmotext, Monsters::MagicCandle, Families::Color(Palette::Cactus), 11, 5);
    spawn_monster(&mut commands, &textures.mrmotext, Monsters::Necromancer, Families::Color(Palette::Red), 26, 5);
    spawn_monster(&mut commands, &textures.mrmotext, Monsters::MrCactus, Families::Color(Palette::Red), 5, 11);
    spawn_weapon(Weapons::Laser, Side::Right, &mut commands, &textures.mrmotext, &mut weapon_changed);

    commands
        .spawn(text("score[000000]", 3, 1, z_pos::GUI))
        .insert(Score(0))
        .insert(SurvivalUI);
    commands
        .spawn(text("life[", 18, 1, z_pos::GUI))
        .insert(SurvivalUI);
    commands
        .spawn(color_text(LIFE_TEXTS[5], 23, 1, z_pos::GUI, Palette::Transparent, Palette::Red))
        .insert(Life(2))
        .insert(SurvivalUI);
    commands
        .spawn(text("]", 28, 1, z_pos::GUI))
        .insert(SurvivalUI);
}

fn increase_score(
    time: Res<Time>,
    mut query: Query<&mut Score>,
) {
    let mut score = query.single_mut();
    score.0 += time.delta().as_millis() as i64;
}

fn update_score(
    mut query: Query<(&Score, &mut text::Text), Changed<Score>>,
) {
    if let Ok((&Score(score), mut text)) = query.get_single_mut() {
        text.text = format!("score:[{:0>6}]", score / 100)
    }
}

fn update_life(
    mut query: Query<(&mut text::Text, &Life), Changed<Life>>,
) {
    if let Ok((mut text, &Life(lives))) = query.get_single_mut() {
        let lives = min(5, max(0, lives)) as usize;
        text.text = LIFE_TEXTS[lives].into();
    }
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

fn cleanup() {}