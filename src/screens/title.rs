use bevy::prelude::*;
use rand::RngCore;

use crate::{choose, GameState};
use crate::choose::Select;
use crate::graphics::background;
use crate::graphics::background::Background;
use crate::graphics::frame::spawn_frame;
use crate::graphics::text::color_text;
use crate::graphics::transition::Transition;
use crate::progress::Progress;
use crate::screens::Textures;
use crate::util::{Palette, Side, z_pos};

pub struct TitlePlugin;

#[derive(Component)]
struct TitleUI;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems((setup, choose::setup).in_schedule(OnEnter(GameState::Title)))
            .add_systems((exit_title, choose::update).in_set(OnUpdate(GameState::Title)))
            .add_systems((cleanup, choose::cleanup).in_schedule(OnExit(GameState::Title)));
    }
}

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    for (t, x, y) in [
        ("Bevy Jam 3", 11, 15),
        ("- normal", 4, 11),
        ("rounds", 5, 9),
        ("with", 5, 8),
        ("upgrade", 5, 7),
        ("breaks", 5, 6),
        ("- hard", 19, 11),
        ("five", 20, 9),
        ("upgrades", 20, 8),
        ("no", 20, 7),
        ("breaks", 20, 6),
    ] {
        commands
            .spawn(color_text(t, x, y, z_pos::BACKGROUND_TEXT, Palette::Transparent, Palette::LightTerracotta))
            .insert(TitleUI);
    }

    spawn_frame(&mut commands, &textures.mrmotext);

    background::spawn_rails(&mut commands, &textures.mrmotext);
    background::spawn_layout(&mut commands, Side::Left, (rand::thread_rng().next_u32() % 8) as isize - 4, &textures.mrmotext);
    background::spawn_layout(&mut commands, Side::Right, (rand::thread_rng().next_u32() % 8) as isize - 4, &textures.mrmotext);
}

fn exit_title(
    mut commands: Commands,
    mut selection: EventReader<Select>,
) {
    for Select(side) in selection.iter() {
        commands.insert_resource(Progress::default());
        // TODO: Different game mode
        commands.insert_resource(Transition::to(GameState::Survival));
    }
}

fn cleanup(
    query: Query<Entity, With<TitleUI>>,
    background: Query<Entity, With<Background>>,
    mut commands: Commands,
) {
    for e in &query { commands.entity(e).despawn_recursive() }
    for e in &background { commands.entity(e).despawn_recursive() }
}
