use bevy::prelude::*;

use crate::GameState;
use crate::graphics::frame::spawn_frame;
use crate::graphics::text;
use crate::loading::Textures;

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

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    spawn_frame(&mut commands, &textures.mrmotext);
}

fn update(
    mut texts: Query<&mut text::Text>,
) {
    let mut text = texts.single_mut();
    let Some((_, score)) = text.text.split_once(" ") else { return };
    text.text = format!("score: {}", score.parse::<usize>().unwrap() + 1)
}

fn cleanup(
) {

}