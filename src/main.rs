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

fn main() {
    App::new()
        .add_plugins(EnginePlugins)
        .add_system_set(SystemSet::on_enter(AppState::GameLoad).with_system(tm_startup))
        .add_system_set(SystemSet::on_update(AppState::GameLoad).with_system(enter_game))
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

fn tm_startup(mut commands: Commands, tiles: Res<SpriteAssets>) {
    let tilemap_size = TilemapSize { x: 500, y: 500 };

    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(tilemap_size);

    // Spawn the elements of the tilemap.
    for x in 0..tilemap_size.x as u32 {
        for y in 0..tilemap_size.y as u32 {
            let tile_pos = TilePos { x, y };
            let tile_index = rand::random::<u32>() % 5;
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile_index),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let grid_size = tile_size.into(); // Not really sure what this is doing?
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: tilemap_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(tiles.tiles1.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&tilemap_size, &grid_size, &map_type, Z_FLOOR),
        ..Default::default()
    });
}
