use bevy::app::App;
use bevy::prelude::*;

use crate::loading::Textures;
use crate::util;
use crate::util::Palette;

pub struct TextPlugin;

impl Plugin for TextPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_texts);
    }
}

#[derive(Component)]
pub struct Text {
    pub text: String,
    pub x: usize,
    pub y: usize,
    pub z: f32,
    children: Vec<Entity>,
}

impl Text {
    pub fn from_str(text: &str, x: usize, y: usize, z: f32) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
            z,
            children: vec![],
        }
    }
}

fn update_texts(
    mut commands: Commands,
    mut texts: ParamSet<(
        Query<(Ref<Text>, Entity)>,
        Query<&mut Text>,
    )>,
    textures: Option<Res<Textures>>,
) {
    let Some(textures) = textures else { return };

    let mut to_update = vec![];
    for (text, id) in &texts.p0() {
        if text.is_changed() {
            to_update.push(id.clone());
        }
    }

    let mut query = texts.p1();
    for id in &to_update {
        let mut text = query.get_mut(*id).unwrap();

        for child in &text.children {
            commands.entity(*child).despawn();
        }
        text.children.clear();

        let mut spawned_entities = vec![];
        for (i, char) in text.text.chars().enumerate() {
            spawned_entities.push(commands.spawn(
                util::sprite(
                    glyph_index(char).unwrap_or(0),
                    text.x + i, text.y, text.z,
                    Palette::BLACK, Palette::WHITE,
                    false, 0,
                    textures.mrmotext.clone()
                )
            ).id());
        }

        text.children.append(&mut spawned_entities);
    }
}

fn glyph_index(c: char) -> Option<usize> {
    match c {
        'a'..='z' => Some(c as usize - 'a' as usize + 897),
        '!'..='_' => Some(c as usize - '!' as usize + 865),
        _ => None,
    }
}