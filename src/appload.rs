use bevy::{
    prelude::*,
    render::texture::{ImageSampler, ImageSettings},
    window::PresentMode,
};
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};

use crate::AppState;
pub struct AppLoadingPlugin;

impl Plugin for AppLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageSettings {
            default_sampler: ImageSampler::nearest_descriptor(),
        })
        .add_loading_state(
            LoadingState::new(AppState::AssetLoad)
                .continue_to_state(AppState::GameLoad)
                .with_collection::<SpriteAssets>(),
        )
        .add_state(AppState::AssetLoad)
        .insert_resource(WindowDescriptor {
            width: 640.0,
            height: 480.0,
            title: "MiniRust".to_string(),
            present_mode: PresentMode::AutoVsync,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins);
    }
}

#[derive(AssetCollection)]
pub struct SpriteAssets {
    #[asset(texture_atlas(tile_size_x = 16., tile_size_y = 16., columns = 4, rows = 1))]
    #[asset(path = "player_move.png")]
    pub player_move: Handle<TextureAtlas>,
    #[asset(path = "tiles.png")]
    pub tiles1: Handle<Image>,
    #[asset(path = "tiles2.png")]
    pub tiles2: Handle<Image>,
}
