use std::cmp::{max, min};

use bevy::prelude::*;

use crate::{GameState, rounds, util};
use crate::characters::monsters::{monster_dies, move_monsters};
use crate::characters::ship::{monsters_kill, Ship, ShipMoveEvent, spawn_ship, update_ship_image, update_ship_name, update_ship_y};
use crate::collision::{add_invincible, collide};
use crate::graphics::background::Background;
use crate::graphics::text;
use crate::graphics::text::{color_text, text};
use crate::graphics::transition::Transition;
use crate::rounds::CurrentRound;
use crate::screens::Textures;
use crate::util::{Palette, Side, z_pos};
use crate::weapons::{monster_looses_life, spawn_weapon, WeaponChanged, Weapons};

pub struct SurvivalPlugin;

impl Plugin for SurvivalPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShipMoveEvent>()
            .add_system(setup.in_schedule(OnEnter(GameState::Survival)))
            .add_systems(
                (update_score, increase_score, update_life, update_ship_image, update_ship_y,
                 update_ship_name, monster_looses_life, monster_dies, move_monsters, rounds::update,
                 monsters_kill.after(add_invincible).after(collide), game_over.after(add_invincible))
                    .in_set(OnUpdate(GameState::Survival))
            )
            .add_system(cleanup.in_schedule(OnExit(GameState::Survival)));
    }
}

#[derive(Component)]
pub struct SurvivalUI;

#[derive(Component)]
pub struct Score(i64);

#[derive(Component)]
pub struct Life(pub(crate) i8);

const LIFE_TEXTS: [&str; 6] = ["°°°°°", "•°°°°", "••°°°", "•••°°", "••••°", "•••••"];


fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
    mut weapon_changed: EventWriter<WeaponChanged>,
) {
    spawn_ship(&mut commands, &textures.mrmotext);
    spawn_weapon(Weapons::Laser, Side::Left, &mut commands, &textures.mrmotext, &mut weapon_changed);
    spawn_weapon(Weapons::Finger, Side::Right, &mut commands, &textures.mrmotext, &mut weapon_changed);

    commands
        .spawn(text("score[000000]", 3, 1, z_pos::GUI))
        .insert(Score(0))
        .insert(SurvivalUI);
    commands
        .spawn(text("life[", 18, 1, z_pos::GUI))
        .insert(SurvivalUI);
    commands
        .spawn(color_text(LIFE_TEXTS[0], 23, 1, z_pos::GUI, Palette::Transparent, Palette::Red))
        .insert(Life(3))
        .insert(SurvivalUI);
    commands
        .spawn(text("]", 28, 1, z_pos::GUI))
        .insert(SurvivalUI);

    // Round
    commands.insert_resource(CurrentRound::new());
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

fn game_over(
    mut commands: Commands,
    lives: Query<&Life, Changed<Life>>,
    ship: Query<&Ship>,
) {
    if let Ok(&Life(lives)) = lives.get_single() {
        if lives <= 0 && ship.get_single().is_ok() {
            commands.insert_resource(Transition::to(GameState::Title).with_delay(util::fight::GAME_OVER_DELAY));
        }
    }
}

fn cleanup(
    mut commands: Commands,
    query: Query<Entity, With<SurvivalUI>>,
    background: Query<Entity, (With<Background>, Without<SurvivalUI>)>,
) {
    for e in &query { commands.entity(e).despawn_recursive(); }
    for e in &background { commands.entity(e).despawn_recursive(); }
}