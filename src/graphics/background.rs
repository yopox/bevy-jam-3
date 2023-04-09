use bevy::prelude::*;

use crate::{GameState, util};
use crate::loading::Textures;
use crate::util::{Palette, size, z_pos};

pub struct BackgroundPlugin;

#[derive(Component)]
struct Background;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(setup.in_schedule(OnEnter(GameState::Survival)))
            .add_system(update_background.in_set(OnUpdate(GameState::Survival)));
    }
}

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
) {
    spawn_rails(&mut commands, &textures.mrmotext);
}

#[derive(Component)]
struct Rail(usize);

fn spawn_rails(
    commands: &mut Commands,
    atlas: &Handle<TextureAtlas>
) {
    for y in 0..size::HEIGHT + 1{
        for x in 0..2 {
            spawn_rail(commands, atlas, x, y);
        }
    }
}

fn spawn_rail(commands: &mut Commands, atlas: &Handle<TextureAtlas>, x: usize, y: usize) {
    commands
        .spawn(util::sprite(
            if rand::random::<f32>() < 0.1 { 299 } else { 331 }, x + 15, y, z_pos::RAILS,
            Palette::Transparent, Palette::Gravel,
            x == 1, 0,
            atlas.clone(),
        ))
        .insert(Rail(x))
        .insert(Background);
}

fn update_background(
    mut commands: Commands,
    mut bg: Query<(&mut Transform, Option<&Rail>, Entity), With<Background>>,
    textures: Res<Textures>,
) {
    for (mut pos, rail, id) in bg.iter_mut() {
        pos.translation.y -= util::background::SPEED;

        if let Some(rail) = rail {
            if pos.translation.y <= -8. {
                commands.entity(id).despawn_recursive();
                spawn_rail(&mut commands, &textures.mrmotext, rail.0, size::HEIGHT - 1);
            }
        } else {
            if pos.translation.y < -120. {
                commands.entity(id).despawn_recursive();
            }
        }
    }
}