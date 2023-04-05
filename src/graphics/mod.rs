use bevy::app::App;
use bevy::prelude::Plugin;

use crate::graphics::text::TextPlugin;

pub mod frame;
pub mod text;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(TextPlugin);
    }
}