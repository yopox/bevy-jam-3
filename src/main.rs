use bevy::prelude::*;
use bevy_text_mode::TextModePlugin;

use crate::graphics::GraphicsPlugin;
use crate::loading::LoadingPlugin;
use crate::survival::SurvivalPlugin;
use crate::util::size;
use crate::util::size::tile_to_f32;
use crate::weapons::WeaponPlugin;

mod util;
mod loading;
mod graphics;
mod survival;
mod weapons;

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
                        size::SCALE * tile_to_f32(size::WIDTH),
                        size::SCALE * tile_to_f32(size::HEIGHT),
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
        .add_plugin(GraphicsPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(SurvivalPlugin)
        .add_plugin(WeaponPlugin)
        .add_startup_system(init)
        .run();
}

fn init(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            scale: Vec3::new(1. / size::SCALE, 1. / size::SCALE, 1.),
            translation: Vec3::new(
                tile_to_f32(size::WIDTH) / 2.,
                tile_to_f32(size::HEIGHT) / 2.,
                100.),
            ..Default::default()
        },
        ..Default::default()
    });
}

#[derive(Bundle, Debug, Default)]
pub struct MainBundle {
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: VisibilityBundle,
}

impl MainBundle {
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        MainBundle {
            transform: Transform::from_xyz(x, y, z),
            ..default()
        }
    }
}