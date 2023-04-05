use bevy::prelude::*;

use crate::GameState;
use crate::graphics::background::spawn_chains;
use crate::graphics::frame::spawn_frame;
use crate::graphics::ship::spawn_ship;
use crate::graphics::text;
use crate::graphics::text::{color_text, text};
use crate::loading::Textures;
use crate::util::{Palette, z_pos};

pub struct SurvivalPlugin;

impl Plugin for SurvivalPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(GameState::Survival)))
            .add_system(
                update
                    .in_set(OnUpdate(GameState::Survival))
            )
            .add_system(cleanup.in_schedule(OnExit(GameState::Survival)));
    }
}

#[derive(Component)]
struct SurvivalUI;

#[derive(Component)]
struct Score;

#[derive(Component)]
struct Life;

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    spawn_frame(&mut commands, &textures.mrmotext);
    spawn_chains(&mut commands, &textures.mrmotext);
    spawn_ship(&mut commands, &textures.mrmotext);

    commands
        .spawn(text("score:000000", 3, 1, z_pos::GUI))
        .insert(Score)
        .insert(SurvivalUI);
    commands
        .spawn(text("lives:", 19, 1, z_pos::GUI))
        .insert(SurvivalUI);
    commands
        .spawn(color_text("****", 25, 1, z_pos::GUI, Palette::BLACK, Palette::RED))
        .insert(Life)
        .insert(SurvivalUI);
}

fn update(
    mut score: Query<&mut text::Text, With<Score>>,
) {
    let mut text = score.single_mut();
    let Some((_, score)) = text.text.split_once(":") else { return };
    text.text = format!("score:{:0>6}", score.parse::<usize>().unwrap() + 1)
}

fn cleanup(
) {

}