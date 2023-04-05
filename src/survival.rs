use std::cmp::{max, min};

use bevy::prelude::*;

use crate::GameState;
use crate::graphics::background::spawn_chains;
use crate::graphics::frame::spawn_frame;
use crate::graphics::ship::spawn_ship;
use crate::graphics::text;
use crate::graphics::text::{color_text, text};
use crate::loading::Textures;
use crate::util::{Palette, z_pos};
use crate::weapons::{Side, spawn_weapon, Weapons};

pub struct SurvivalPlugin;

impl Plugin for SurvivalPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(GameState::Survival)))
            .add_systems(
                (update_score, increase_score, update_life)
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

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    spawn_frame(&mut commands, &textures.mrmotext);
    spawn_chains(&mut commands, &textures.mrmotext);
    spawn_ship(&mut commands, &textures.mrmotext);
    spawn_weapon(Weapons::Finger, Side::Left, &mut commands, &textures.mrmotext);
    spawn_weapon(Weapons::Finger, Side::Right, &mut commands, &textures.mrmotext);

    commands
        .spawn(text("score[000000]", 3, 1, z_pos::GUI))
        .insert(Score(0))
        .insert(SurvivalUI);
    commands
        .spawn(text("life[", 18, 1, z_pos::GUI))
        .insert(SurvivalUI);
    commands
        .spawn(color_text("****", 23, 1, z_pos::GUI, Palette::BLACK, Palette::RED))
        .insert(Life(5))
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
    mut query: Query<(&Score, &mut text::Text)>,
) {
    let (&Score(score), mut text) = query.single_mut();
    text.text = format!("score[{:0>6}]", score / 100)
}

const LIFE_TEXTS: [&str; 6] = ["", "*", "**", "***", "****", "*****"];

fn update_life(
    mut query: Query<(&mut text::Text, &Life), Changed<Life>>,
) {
    for (mut text, &Life(lives)) in query.iter_mut() {
        let lives = min(5, max(0, lives)) as usize;
        text.text = LIFE_TEXTS[lives].into();
    }
}

fn cleanup(
) {

}