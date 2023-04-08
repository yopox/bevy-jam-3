use bevy::prelude::*;

use crate::GameState;
use crate::graphics::text;
use crate::util::{Palette, z_pos};

pub struct TitlePlugin;

#[derive(Component)]
struct TitleUI;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(GameState::Title)))
            .add_system(exit_title.in_set(OnUpdate(GameState::Title)))
            .add_system(cleanup.in_schedule(OnExit(GameState::Title)));
    }
}

fn setup(
    mut commands: Commands
) {
    commands.spawn(text::from_middle("Game Jame 3", 0, 2, z_pos::GUI, Palette::Transparent, Palette::LightTerracotta))
        .insert(TitleUI);
    commands.spawn(text::from_middle("Press any key to continue.", 0, -6, z_pos::GUI, Palette::Transparent, Palette::LightTerracotta))
        .insert(TitleUI);
}

fn exit_title(
    keys: Res<Input<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.any_just_pressed([KeyCode::Space, KeyCode::Return]) {
        next_state.set(GameState::Survival)
    }
}

fn cleanup(
    query: Query<Entity, With<TitleUI>>,
    mut commands: Commands,
) {
    for e in query.iter() {
        commands.entity(e).despawn_recursive()
    }
}
