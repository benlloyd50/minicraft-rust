mod assetload;
mod camera;
mod debug;
mod engine;
mod inventory;
mod item;
mod map_generation;
mod player;
mod sound_event;
mod states;
pub use assetload::FontAssets;
pub use assetload::SpriteAssets;
use bevy_ecs_tilemap::TilemapPlugin;
pub use camera::CameraPlugin;
pub use engine::EnginePlugins;
pub use inventory::InventoryPlugin;
pub use item::ItemPlugin;
pub use map_generation::MapGenerationPlugin;
pub use player::Interact;
pub use player::PlayerPlugin;
pub use sound_event::GameSoundPlugin;
pub use states::AppState;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(EnginePlugins)
        .add_system_set(SystemSet::on_update(AppState::GameLoad).with_system(enter_game))
        .add_plugin(TilemapPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(ItemPlugin)
        .add_plugin(InventoryPlugin)
        .add_plugin(GameSoundPlugin)
        .add_plugin(MapGenerationPlugin)
        .run();
}

fn enter_game(mut state: ResMut<State<AppState>>) {
    if state.current() == &AppState::GameLoad {
        state
            .set(AppState::InGame)
            .expect("Failed to change states");
    }
}
