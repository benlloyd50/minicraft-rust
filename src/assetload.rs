use bevy::prelude::*;
use bevy_asset_loader::prelude::{AssetCollection, LoadingState, LoadingStateAppExt};

use crate::AppState;
pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::AssetLoad)
                .continue_to_state(AppState::GameLoad)
                .with_collection::<SpriteAssets>(),
        );
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
