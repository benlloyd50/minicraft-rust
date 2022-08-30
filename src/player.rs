use crate::SpriteAssets;

use super::AppState;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

const Z_PLAYER: f32 = 40.;
const PLAYER_SPEED: f32 = 40.0;
// const TIME_STEP: f32 = 1.0 / 60.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(load_player))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(player_input)
                    .with_system(direction_animation),
            );
    }
}

#[derive(Component, Inspectable)]
pub struct Player;

//TODO: Consider moving these Components into a shared class
#[derive(Component)]
enum State {
    Idle,
    Moving,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn load_player(mut commands: Commands, sprites: Res<SpriteAssets>) {
    let _player_entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: sprites.player_move.clone(),
            transform: Transform::from_xyz(10., 0., Z_PLAYER),
            ..Default::default()
        })
        .insert(Direction::Up)
        .insert(Player)
        .insert(AnimationTimer(Timer::from_seconds(0.175, true)))
        .insert(State::Idle)
        .id();
}

fn direction_animation(
    time: Res<Time>,
    mut q: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Direction,
        &mut State,
    )>,
) {
    for (mut timer, mut sprite, dir, state) in &mut q {
        match *state {
            State::Idle => {
                return;
            }
            State::Moving => {}
        }
        timer.tick(time.delta());
        if timer.just_finished() {
            match *dir {
                Direction::Up => {
                    sprite.index = 1;
                    sprite.flip_x = !sprite.flip_x;
                }
                Direction::Down => {
                    sprite.index = 0;
                    sprite.flip_x = !sprite.flip_x;
                }
                Direction::Right => {
                    sprite.index = if sprite.index == 2 { 3 } else { 2 };
                    sprite.flip_x = false;
                }
                Direction::Left => {
                    sprite.index = if sprite.index == 2 { 3 } else { 2 };
                    sprite.flip_x = true;
                }
            }
        }
    }
}

//TODO: We should use some sort of move function rather than this
fn player_input(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Direction, &mut State, &Player)>,
) {
    let (mut p_transform, mut p_direction, mut state, _) = query.single_mut();
    let mut input = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        input.x -= 1.0;
        *p_direction = Direction::Left;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input.x += 1.0;
        *p_direction = Direction::Right;
    }
    if keyboard_input.pressed(KeyCode::W) {
        input.y += 1.0;
        *p_direction = Direction::Up;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input.y -= 1.0;
        *p_direction = Direction::Down;
    }

    move_entity(
        &mut *p_transform,
        input.x * PLAYER_SPEED * time.delta_seconds(),
        input.y * PLAYER_SPEED * time.delta_seconds(),
    );

    if input != Vec2::ZERO {
        *state = State::Moving
    } else {
        *state = State::Idle
    }
}

//Moves the transform component
fn move_entity(transform: &mut Transform, dx: f32, dy: f32) {
    transform.translation.x += dx;
    transform.translation.y += dy;
}
