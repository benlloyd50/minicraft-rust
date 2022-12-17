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
pub use camera::CameraPlugin;
pub use engine::EnginePlugins;
pub use inventory::InventoryPlugin;
pub use item::ItemPlugin;
pub use map_generation::MapGenerationPlugin;
pub use player::PlayerPlugin;
pub use player::{Interact, PlayerEntity};
pub use sound_event::GameSoundPlugin;
pub use states::AppState;

use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(EnginePlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::ZERO, // gravity does not exist in a 2.5d world
            ..Default::default()
        })
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_system_set(SystemSet::on_update(AppState::InGame).with_system(display_events))
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

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    _player: Res<PlayerEntity>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
