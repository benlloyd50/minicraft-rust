use crate::SpriteAssets;

use super::AppState;
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

const Z_PLAYER: f32 = 40.;
const PLAYER_SPEED: f32 = 40.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(load_player))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(player_input.label(PSystems::Input))
                    .with_system(move_player.label(PSystems::Movement).after(PSystems::Input))
                    .with_system(direction_animation),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum PSystems {
    Input,
    Movement,
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
#[derive(Component, Inspectable)]
pub struct Player;

#[derive(Component)]
enum PlayerState {
    Idle,
    Moving,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component)]
pub struct InputCapture {
    movement: Vec2,
}

fn load_player(mut commands: Commands, sprites: Res<SpriteAssets>) {
    let _player_entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: sprites.player_move.clone(),
            transform: Transform::from_xyz(10., 0., Z_PLAYER),
            ..Default::default()
        })
        .insert(Player)
        .insert(AnimationTimer(Timer::from_seconds(0.175, true)))
        .insert(PlayerState::Idle)
        .insert(InputCapture {
            movement: Vec2::ZERO,
        })
        .insert(Direction::Down)
        .id();
}

fn direction_animation(
    time: Res<Time>,
    mut q: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &mut Direction,
        &mut PlayerState,
    )>,
) {
    for (mut timer, mut sprite, dir, state) in &mut q {
        match *state {
            PlayerState::Idle => {
                return;
            }
            PlayerState::Moving => {}
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

fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut q: Query<(&mut InputCapture, &mut Direction), With<Player>>,
) {
    let (mut input, mut dir) = q.single_mut();
    input.movement = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        input.movement.x -= 1.0;
        *dir = Direction::Left;
    }
    if keyboard_input.pressed(KeyCode::D) {
        input.movement.x += 1.0;
        *dir = Direction::Right;
    }
    if keyboard_input.pressed(KeyCode::W) {
        input.movement.y += 1.0;
        *dir = Direction::Up;
    }
    if keyboard_input.pressed(KeyCode::S) {
        input.movement.y -= 1.0;
        *dir = Direction::Down;
    }
}

fn move_player(
    time: Res<Time>,
    mut q: Query<(&mut Transform, &mut PlayerState, &InputCapture), With<Player>>,
) {
    let (mut transform, mut state, input_val) = q.single_mut();
    transform.translation.x += input_val.movement.x * PLAYER_SPEED * time.delta_seconds();
    transform.translation.y += input_val.movement.y * PLAYER_SPEED * time.delta_seconds();

    if input_val.movement != Vec2::ZERO {
        *state = PlayerState::Moving;
    } else {
        *state = PlayerState::Idle;
    }
}
