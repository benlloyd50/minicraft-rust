use bevy::{prelude::*, render::texture::ImageSettings, window::PresentMode};
use bevy_ecs_tilemap::prelude::*;

const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
// pub const RESOLUTION: f32 = 16.0 / 9.0;
const PLAYER_SPEED: f32 = 40.0;
const TIME_STEP: f32 = 1.0 / 60.0;

const Z_CAM: f32 = 100.;
const Z_PLAYER: f32 = 40.;
const Z_FLOOR: f32 = 0.;

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
        .add_plugins(DefaultPlugins)
        .add_plugin(TilemapPlugin)
        .add_startup_system(tm_startup)
        .add_state(AppState::InGame)
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(load_player_with_cam))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(camera_follow_player)
                .with_system(player_input)
                .with_system(player_anim),
        )
        .add_system(swap_texture_or_hide)
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

fn camera_follow_player(
    mut cameras: Query<(&mut Transform, &Camera2d), Without<Player>>,
    players: Query<(&mut Transform, &Player), Without<Camera2d>>,
) {
    for (player, _) in players.iter() {
        for (mut cam, _) in cameras.iter_mut() {
            cam.translation.x = player.translation.x;
            cam.translation.y = player.translation.y;
        }
    }
}

fn load_player_with_cam(
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
            transform: Transform::from_xyz(10., 0., Z_PLAYER),
            ..Default::default()
        })
        .insert(Direction::Up)
        .insert(Player)
        .insert(AnimationTimer(Timer::from_seconds(0.175, true)))
        .insert(State::Idle)
        .id();

    let camera_entity = commands
        .spawn_bundle(Camera2dBundle::new_with_far(Z_CAM))
        .id();
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

    p_transform.translation.x += input.x * PLAYER_SPEED * TIME_STEP;
    p_transform.translation.y += input.y * PLAYER_SPEED * TIME_STEP;

    if input != Vec2::ZERO {
        *state = State::Moving
    } else {
        *state = State::Idle
    }
}

fn tm_startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture_handle: Handle<Image> = asset_server.load("tiles.png");
    let tilemap_size = TilemapSize { x: 100, y: 100 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(tilemap_size);

    // Spawn the elements of the tilemap.
    for x in 0..100u32 {
        for y in 0..100u32 {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, Some(tile_entity));
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };

    commands
        .entity(tilemap_entity)
        .insert_bundle(TilemapBundle {
            grid_size: TilemapGridSize { x: 16.0, y: 16.0 },
            size: tilemap_size,
            storage: tile_storage,
            texture: TilemapTexture(texture_handle),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                Z_FLOOR,
            ),
            ..Default::default()
        });
}

fn swap_texture_or_hide(
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut TilemapTexture, &mut Visibility)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let texture_handle_a: Handle<Image> = asset_server.load("tiles.png");
        let texture_handle_b: Handle<Image> = asset_server.load("tiles2.png");
        for (mut tilemap_tex, _) in &mut query {
            if &tilemap_tex.0 == &texture_handle_a {
                tilemap_tex.0 = texture_handle_b.clone();
            } else {
                tilemap_tex.0 = texture_handle_a.clone();
            }
        }
    }
    if keyboard_input.just_pressed(KeyCode::H) {
        for (_, mut visibility) in &mut query {
            if visibility.is_visible {
                visibility.is_visible = false;
            } else {
                visibility.is_visible = true;
            }
        }
    }
}
