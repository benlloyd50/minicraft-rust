use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};

const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
// pub const RESOLUTION: f32 = 16.0 / 9.0;
const PLAYER_SPEED: f32 = 25.0;
const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused,
}

fn main() {
    let height = 120.0;
    let width = 160.0;
    let scale = 3.0;

    let _app = App::new()
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(CLEAR))
        .insert_resource(WindowDescriptor {
            width: width * scale,
            height: height * scale,
            title: "MiniRust".to_string(),
            present_mode: PresentMode::AutoVsync,
            resizable: false,
            ..Default::default()
        })
        .add_state(AppState::InGame)
        .add_plugins(DefaultPlugins)
        .add_system_set(
            SystemSet::on_enter(AppState::InGame).with_system(load_player_with_follow_camera),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                // .with_system(camera_follow_player)
                .with_system(player_input)
                .with_system(player_anim),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
enum State {
    Idle,
    Moving,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// struct PlayerData {
//     player_entity: Entity,
//     camera_entity: Entity,
// }

// fn camera_follow_player(
//     mut cameras: Query<&mut Transform, With<Camera2d>>,
//     players: Query<&Transform, With<Player>>,
// ) {
//     for player in players.iter() {
//         for mut cam in cameras.iter_mut() {
//             cam.translation.x = player.translation.x;
//             cam.translation.y = player.translation.y;
//         }
//     }
// }

fn load_player_with_follow_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("player_move.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 4, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_entity = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_xyz(10., 0., 0.),
            ..Default::default()
        })
        .insert(Direction::Up)
        .insert(Player)
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)))
        .insert(State::Idle)
        .id();

    let camera_entity = commands.spawn_bundle(Camera2dBundle::default()).id();
    // commands.insert_resource(PlayerData {
    //     player_entity,
    //     camera_entity,
    // });
}

fn player_anim(
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
                    println!("UP");
                    sprite.index = 1;
                    sprite.flip_x = !sprite.flip_x;
                }
                Direction::Down => {
                    println!("DOWN");
                    sprite.index = 0;
                    sprite.flip_x = !sprite.flip_x;
                }
                Direction::Right => {
                    println!("RIGHT");
                    sprite.index = if sprite.index == 2 { 3 } else { 2 };
                    sprite.flip_x = false;
                }
                Direction::Left => {
                    println!("LEFT");
                    sprite.index = if sprite.index == 2 { 3 } else { 2 };
                    sprite.flip_x = true;
                }
            }
        }
    }
}

fn player_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Direction, &mut State), With<Player>>,
) {
    let (mut p_transform, mut p_direction, mut state) = query.single_mut();
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

    p_transform.translation.x += input.x * PLAYER_SPEED * TIME_STEP;
    p_transform.translation.y += input.y * PLAYER_SPEED * TIME_STEP;

    if input != Vec2::ZERO {
        *state = State::Moving
    } else {
        *state = State::Idle
    }
}
