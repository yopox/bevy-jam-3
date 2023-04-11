use bevy::app::App;
use bevy::prelude::Plugin;

pub use loading::Textures;

use crate::screens::loading::LoadingPlugin;
use crate::screens::survival::SurvivalPlugin;
use crate::screens::title::TitlePlugin;

mod loading;
pub mod survival;
mod title;

pub struct ScreensPlugin;

impl Plugin for ScreensPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(LoadingPlugin)
            .add_plugin(TitlePlugin)
            .add_plugin(SurvivalPlugin)
        ;
    }
}