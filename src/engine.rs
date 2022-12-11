use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_kira_audio::AudioPlugin;

use crate::assetload::AssetLoadPlugin;
use crate::debug::DebugPlugin;

pub struct EnginePlugins;

impl PluginGroup for EnginePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(DefaultPluginsWithImage)
            .add(AssetLoadPlugin)
            .add(AudioPlugin)
            .add(DebugPlugin)
    }
}

struct DefaultPluginsWithImage;

impl Plugin for DefaultPluginsWithImage {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        width: 640.0,
                        height: 480.0,
                        title: "MiniCraft [Rust]".to_string(),
                        present_mode: PresentMode::AutoVsync,
                        resizable: false,
                        ..Default::default()
                    },
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        );
    }
}
