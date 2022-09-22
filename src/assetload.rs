use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::AppState;
pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::AssetLoad)
                .continue_to_state(AppState::GameLoad)
                .with_collection::<SpriteAssets>()
                .with_collection::<SoundAssets>()
                .with_collection::<FontAssets>(),
        )
        .add_state(AppState::AssetLoad);
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
    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 14, rows = 2))]
    #[asset(path = "test_items.png")]
    pub items: Handle<TextureAtlas>,
    #[asset(path = "menu.png")]
    pub menu: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct SoundAssets {
    #[asset(path = "sounds/pickup.ogg")]
    pub item_pickup: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/CelticTime.ttf")]
    pub celtic: Handle<Font>,
}
