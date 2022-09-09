use bevy::app::PluginGroupBuilder;
use bevy::prelude::*;
use bevy::render::texture::{ImageSampler, ImageSettings};
use bevy::window::PresentMode;
use bevy_kira_audio::AudioPlugin;

use crate::assetload::AssetLoadPlugin;
use crate::debug::DebugPlugin;

pub struct EnginePlugins;

impl PluginGroup for EnginePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(DefaultPluginsWithImage)
            .add(AssetLoadPlugin)
            .add(AudioPlugin)
            .add(DebugPlugin);
    }
}

struct DefaultPluginsWithImage;

impl Plugin for DefaultPluginsWithImage {
    fn build(&self, app: &mut App) {
        app.insert_resource(ImageSettings {
            default_sampler: ImageSampler::nearest_descriptor(),
        })
        .insert_resource(WindowDescriptor {
            width: 640.0,
            height: 480.0,
            title: "MiniCraft [Rust]".to_string(),
            present_mode: PresentMode::AutoVsync,
            resizable: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins);
    }
}
