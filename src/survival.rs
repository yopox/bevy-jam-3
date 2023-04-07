use std::cmp::{max, min};

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;

use crate::GameState;
use crate::graphics::background::spawn_rails;
use crate::graphics::frame::spawn_frame;
use crate::graphics::ship::{ShipMoveEvent, spawn_ship, update_ship_image, update_ship_name, update_ship_y};
use crate::graphics::text;
use crate::graphics::text::{color_text, text};
use crate::loading::Textures;
use crate::util::{Palette, z_pos};
use crate::weapons::{Side, spawn_weapon, WeaponChanged, Weapons};

pub struct SurvivalPlugin;

impl Plugin for SurvivalPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ShipMoveEvent>()
            .add_system(setup.in_schedule(OnEnter(GameState::Survival)))
            .add_systems(
                (update_score, increase_score, update_life, keyboard_dispatcher, update_ship_image, update_ship_y, update_ship_name)
                    .in_set(OnUpdate(GameState::Survival))
            )
            .add_system(cleanup.in_schedule(OnExit(GameState::Survival)));
    }
}

#[derive(Component)]
struct SurvivalUI;

#[derive(Component)]
struct Score(i64);

#[derive(Component)]
struct Life(i8);

const LIFE_TEXTS: [&str; 6] = ["", "*", "**", "***", "****", "*****"];


fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
    mut weapon_changed: EventWriter<WeaponChanged>,
) {
    spawn_frame(&mut commands, &textures.mrmotext);
    spawn_rails(&mut commands, &textures.mrmotext);
    spawn_ship(&mut commands, &textures.mrmotext);
    spawn_weapon(Weapons::Finger, Side::Left, &mut commands, &textures.mrmotext, &mut weapon_changed);
    // spawn_weapon(Weapons::Finger, Side::Right, &mut commands, &textures.mrmotext, &mut weapon_changed);

    commands
        .spawn(text("score[000000]", 3, 1, z_pos::GUI))
        .insert(Score(0))
        .insert(SurvivalUI);
    commands
        .spawn(text("life[", 18, 1, z_pos::GUI))
        .insert(SurvivalUI);
    commands
        .spawn(color_text(LIFE_TEXTS[5], 23, 1, z_pos::GUI, Palette::Black, Palette::Red))
        .insert(Life(5))
        .insert(SurvivalUI);
    commands
        .spawn(text("]", 28, 1, z_pos::GUI))
        .insert(SurvivalUI);
}

fn increase_score(
    time: Res<Time>,
    mut query: Query<&mut Score>,
) {
    let mut score = query.single_mut();
    score.0 += time.delta().as_millis() as i64;
}

fn update_score(
    mut query: Query<(&Score, &mut text::Text), Changed<Score>>,
) {
    if let Ok((&Score(score), mut text)) = query.get_single_mut() {
        text.text = format!("score:[{:0>6}]", score / 100)
    }
}

fn update_life(
    mut query: Query<(&mut text::Text, &Life), Changed<Life>>,
) {
    if let Ok((mut text, &Life(lives))) = query.get_single_mut() {
        let lives = min(5, max(0, lives)) as usize;
        text.text = LIFE_TEXTS[lives].into();
    }
}

pub fn keyboard_dispatcher(
    mut event_reader: EventReader<KeyboardInput>,
    mut ship_event_writer: EventWriter<ShipMoveEvent>,
) {
    let mut ship_moved = 0;
    for event in event_reader.iter() {
        match event.key_code {
            Some(KeyCode::Up) => ship_moved += 1,
            Some(KeyCode::Down) => ship_moved -= 1,
            _ => ()
        }
    }
    if ship_moved != 0 {
        ship_event_writer.send(ShipMoveEvent(ship_moved));
    }
}

fn cleanup(
) {

}