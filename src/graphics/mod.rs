use bevy::app::App;
use bevy::prelude::Plugin;

use crate::graphics::animation::AnimationPlugin;
use crate::graphics::background::BackgroundPlugin;
use crate::graphics::text::TextPlugin;

pub mod frame;
pub mod text;
pub mod background;
pub mod ship;
pub mod tiles;
pub mod monsters;
pub mod sprites;
mod background_sprites;
mod animation;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(tiles::flip)
            .add_plugin(TextPlugin)
            .add_plugin(BackgroundPlugin)
            .add_plugin(AnimationPlugin);
    }
}