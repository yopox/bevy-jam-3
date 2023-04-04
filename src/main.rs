use bevy::prelude::*;
use bevy_text_mode::TextModePlugin;
use crate::loading::LoadingPlugin;
use crate::survival::SurvivalPlugin;
use crate::util::size;

mod util;
mod loading;
mod graphics;
mod survival;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Loading,
    Title,
    Survival,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hex("ffffff").unwrap()))
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: (
                        size::SCALE * size::TILE_SIZE * size::WIDTH as f32,
                        size::SCALE * size::TILE_SIZE * size::HEIGHT as f32
                    ).into(),
                    title: "bevy-jam-3".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    ..default()
                }),
                ..default()
            })
        )
        .add_state::<GameState>()
        .add_plugin(TextModePlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(SurvivalPlugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(1. / size::SCALE, 1. / size::SCALE, 1.),
            translation: Vec3::new(
                size::WIDTH as f32 * size::TILE_SIZE / 2.,
                size::HEIGHT as f32 * size::TILE_SIZE / 2.,
                100.),
            ..Default::default()
        },
        ..Default::default()
    });
}