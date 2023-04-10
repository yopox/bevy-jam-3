use bevy::app::App;
use bevy::input::Input;
use bevy::prelude::*;

use crate::graphics::text;
use crate::graphics::text::text;
use crate::loading::Textures;
use crate::util;
use crate::util::{Palette, Side, sprite, z_pos};

pub struct ChoosePlugin;

impl Plugin for ChoosePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<Select>();
    }
}

#[derive(Resource)]
pub struct Choose(pub Option<Side>);

pub struct Select(pub Side);

#[derive(Component)]
pub struct SelectionText;

#[derive(Component)]
pub struct ChooseUI;

pub fn setup(
    mut commands: Commands,
) {
    commands.insert_resource(Choose(None));

    commands
        .spawn(text(util::choose::TEXT_SELECT, 4, 1, z_pos::GUI))
        .insert(SelectionText)
        .insert(ChooseUI);
}

pub fn update(
    mut commands: Commands,
    mut choose: Option<ResMut<Choose>>,
    mut select: EventWriter<Select>,
    mut text: Query<&mut text::Text, With<SelectionText>>,
    frame: Query<Entity, With<Border>>,
    keys: Res<Input<KeyCode>>,
    textures: Res<Textures>,
) {
    let Some(mut choose) = choose else { return; };

    for (key, side, side_text) in [
        (KeyCode::Left, Side::Left, util::choose::TEXT_CHOOSE_LEFT),
        (KeyCode::Right, Side::Right, util::choose::TEXT_CHOOSE_RIGHT)
    ] {
        if keys.just_pressed(key) {
            if let Some(chosen_side) = choose.0 {
                if chosen_side == side {
                    select.send(Select(side));
                    return;
                }
                else { choose.0 = Some(side); }
            } else { choose.0 = Some(side); }

            for id in &frame {
                commands.entity(id).despawn_recursive();
            }

            spawn_border(&mut commands, &textures.mrmotext, side);

            text.get_single_mut().unwrap().text = side_text.to_string();
        }
    }
}

#[derive(Component)]
pub struct Border(usize, usize);

fn spawn_border(
    commands: &mut Commands,
    atlas: &Handle<TextureAtlas>,
    side: Side,
) {
    let last_x = util::choose::BORDER_WIDTH - 1;
    let last_y = util::choose::BORDER_HEIGHT - 1;
    for y in 0..=last_y {
        for x in 0..=last_x {
            let Some((tile, rotation)) = (match (x, y) {
                (0, 0) => Some((156, 3)),
                (0, y) if y == last_y => Some((156, 0)),
                (x, 0) if x == last_x => Some((156, 2)),
                (x, y) if x == last_x && y == last_y => Some((156, 1)),
                (0, _) => Some((57, 0)),
                (_, 0) => Some((90, 0)),
                (x, _) if x == last_x => Some((59, 0)),
                (_, y) if y == last_y => Some((26, 0)),
                _ => None,
            }) else { continue };

            let side_y = if side == Side::Right { util::choose::SIDE_Y } else { 0 };
            commands
                .spawn(sprite(
                    tile, x + util::choose::BORDER_X + side_y, y + util::choose::BORDER_Y, z_pos::CHOOSE_BORDER,
                    Palette::Transparent, Palette::Lava,
                    false, rotation, atlas.clone()
                ))
                .insert(Border(x, y))
                .insert(ChooseUI);
        }
    }
}

pub fn cleanup(
    mut commands: Commands,
    ui: Query<Entity, With<ChooseUI>>,
) {
    for id in &ui {
        commands.entity(id).despawn_recursive();
    }
}