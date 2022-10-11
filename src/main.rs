mod assetload;
mod camera;
mod debug;
mod engine;
mod inventory;
mod item;
mod player;
mod sound_event;
mod states;
pub use assetload::FontAssets;
pub use assetload::SpriteAssets;
pub use camera::CameraPlugin;
pub use engine::EnginePlugins;
pub use inventory::InventoryPlugin;
pub use item::ItemPlugin;
pub use player::Interact;
pub use player::PlayerPlugin;
pub use sound_event::GameSoundPlugin;
pub use states::AppState;

use bevy::prelude::*;

use bevy_ecs_tilemap::prelude::*;

const Z_FLOOR: f32 = 0.;
const Z_UI: f32 = 80.;

fn main() {
    App::new()
        .add_plugins(EnginePlugins)
        .add_system_set(
            SystemSet::on_enter(AppState::GameLoad)
                .with_system(tm_startup)
                .with_system(ui_startup)
                .with_system(font_render_startup),
        )
        .add_system_set(SystemSet::on_update(AppState::GameLoad).with_system(enter_game))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(swap_texture_or_hide)
                .with_system(toggle_ui_menu)
                .after(Interact::Caller),
        )
        .add_plugin(TilemapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ItemPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(GameSoundPlugin)
        .run();
}

fn enter_game(mut state: ResMut<State<AppState>>) {
    if state.current() == &AppState::GameLoad {
        state
            .set(AppState::InGame)
            .expect("Failed to change states");
    }
}

#[derive(Component)]
// Maybe we should attach a player id for multiplayer?
pub struct PlayerMenu;

#[derive(Component)]
pub struct FixedCamera;

fn ui_startup(mut commands: Commands, elements: Res<SpriteAssets>) {
    // This creates a menu that is fixed to the camera and is attached to the player
    commands
        .spawn_bundle(SpriteBundle {
            texture: elements.menu.clone(),
            transform: Transform::from_xyz(-30.0, -50.0, Z_UI),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(PlayerMenu)
        .insert(FixedCamera);
}

fn toggle_ui_menu(
    keeb_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Visibility, &mut Transform), With<FixedCamera>>,
    cam: Query<(&Camera, &Transform), Without<FixedCamera>>,
) {
    let mut menu = query.single_mut();
    let (_, cam) = cam.single();

    menu.1.translation = Vec3::new(cam.translation.x - 30.0, cam.translation.y - 50.0, Z_UI);

    if keeb_input.just_pressed(KeyCode::Tab) {
        menu.0.is_visible = !menu.0.is_visible;
    }
}

fn tm_startup(mut commands: Commands, tiles: Res<SpriteAssets>) {
    let tilemap_size = TilemapSize { x: 10, y: 10 };

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
    for x in 0..tilemap_size.x as u32 {
        for y in 0..tilemap_size.y as u32 {
            let tile_pos = TilePos { x, y };
            let tile_index = rand::random::<u32>() % 5;
            let tile_entity = commands
                .spawn()
                .insert_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture: TileTexture(tile_index),
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
            texture: TilemapTexture(tiles.tiles1.clone()),
            tile_size,
            transform: bevy_ecs_tilemap::helpers::get_centered_transform_2d(
                &tilemap_size,
                &tile_size,
                Z_FLOOR,
            ),
            ..Default::default()
        });
}

fn font_render_startup(mut commands: Commands, font: Res<FontAssets>) {
    let monogram = font.monogram.clone();
    let text_style = TextStyle {
        font: monogram,
        font_size: 24.0,
        color: Color::BLACK,
    };
    commands.spawn_bundle(Text2dBundle {
        text: Text::from_section("This is a test.", text_style.clone())
            .with_alignment(TextAlignment::CENTER),
        transform: Transform::from_xyz(40.0, 180.0, Z_UI),
        ..default()
    });
}

fn swap_texture_or_hide(
    sprites: Res<SpriteAssets>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut TilemapTexture, &mut Visibility)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        for (mut tilemap_tex, _) in &mut query {
            if tilemap_tex.0 == sprites.tiles1 {
                tilemap_tex.0 = sprites.tiles2.clone();
            } else {
                tilemap_tex.0 = sprites.tiles1.clone();
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
