use bevy::prelude::*;
use rand::prelude::IteratorRandom;
use rand::RngCore;
use strum::IntoEnumIterator;

use crate::{GameState, MainBundle, util};
use crate::graphics::background_sprites::Layouts;
use crate::graphics::sprites;
use crate::loading::Textures;
use crate::util::{Palette, Side, size, z_pos};
use crate::util::size::tile_to_f32;

pub struct BackgroundPlugin;

#[derive(Component)]
struct Background;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(BackgroundTimer(0, 0))
            .add_system(setup.in_schedule(OnEnter(GameState::Survival)))
            .add_system(update_background.in_set(OnUpdate(GameState::Survival)));
    }
}

fn setup(
    mut commands: Commands,
    textures: Res<Textures>,
    mut timer: ResMut<BackgroundTimer>,
) {
    spawn_rails(&mut commands, &textures.mrmotext);

    for dy in [0, util::background::LAYOUT_HEIGHT] {
        let layout = Layouts::iter().choose(&mut rand::thread_rng()).unwrap();
        timer.0 = spawn_layout(&mut commands, Side::Left, layout, dy, &textures.mrmotext);
        let layout = Layouts::iter().choose(&mut rand::thread_rng()).unwrap();
        timer.1 = spawn_layout(&mut commands, Side::Right, layout, dy, &textures.mrmotext);
    }
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
    let mut bundle = util::sprite(
            if rand::random::<f32>() < 0.1 { 299 } else { 331 }, x + 15, y, z_pos::RAILS,
            Palette::Transparent, Palette::Gravel,
            x == 1, 0,
            atlas.clone(),
        );
    bundle.sprite.alpha = util::background::ALPHA;

    commands
        .spawn(bundle)
        .insert(Rail(x))
        .insert(Background);
}

#[derive(Resource)]
struct BackgroundTimer(isize, isize);

fn update_background(
    mut commands: Commands,
    mut bg: Query<(&mut Transform, Option<&Rail>, Entity), With<Background>>,
    mut timer: ResMut<BackgroundTimer>,
    textures: Res<Textures>,
) {
    // Update timer
    timer.0 -= 1;
    timer.1 -= 1;
    info!("Timer: {}, {}", timer.0, timer.1);
    if timer.0 <= 0 {
        let layout = Layouts::iter().choose(&mut rand::thread_rng()).unwrap();
        timer.0 = spawn_layout(&mut commands, Side::Left, layout, util::background::LAYOUT_HEIGHT, &textures.mrmotext);
    } else if timer.1 <= 0 {
        let layout = Layouts::iter().choose(&mut rand::thread_rng()).unwrap();
        timer.1 = spawn_layout(&mut commands, Side::Right, layout, util::background::LAYOUT_HEIGHT, &textures.mrmotext);
    }

    // Move and despawn entities
    for (mut pos, rail, id) in bg.iter_mut() {
        pos.translation.y -= util::background::SPEED;

        if let Some(rail) = rail {
            if pos.translation.y <= -8. {
                commands.entity(id).despawn_recursive();
                spawn_rail(&mut commands, &textures.mrmotext, rail.0, size::HEIGHT);
            }
        } else {
            if pos.translation.y < -120. {
                commands.entity(id).despawn_recursive();
            }
        }
    }
}

fn spawn_layout(
    commands: &mut Commands,
    side: Side,
    layout: Layouts,
    dy: usize,
    atlas: &Handle<TextureAtlas>,
) -> isize {
    let size = util::background::LAYOUT_HEIGHT;
    let offset_y = rand::thread_rng().next_u32() % 6;

    for (element, x, y) in layout.get_elements() {
        commands
            .spawn(MainBundle::from_xyz(
                tile_to_f32(x + if side == Side::Left { 2 } else { 17 }),
                tile_to_f32(y + 3 + dy + offset_y as usize),
                z_pos::BACKGROUND
            ))
            .insert(Background)
            .with_children(|builder| {
                for &(tile_x, tile_y, i, bg, fg, flip, rotation) in element.get_sprite().iter() {
                    let mut bundle = util::sprite(
                            i, tile_x, tile_y, 0.,
                            sprites::RTEMO_PALETTE[bg], sprites::RTEMO_PALETTE[fg],
                            flip, rotation,
                            atlas.clone(),
                        );
                    bundle.sprite.alpha = util::background::ALPHA;
                    builder.spawn(bundle);
                }
            });
    }

    return (tile_to_f32(size + offset_y as usize) / util::background::SPEED) as isize;
}