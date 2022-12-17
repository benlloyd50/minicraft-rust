use bevy::{
    diagnostic::LogDiagnosticsPlugin,
    // diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::*,
};
use bevy_inspector_egui::{widgets::InspectorQuerySingle, Inspectable, InspectorPlugin};

use crate::player::Player;

pub struct DebugPlugin;

#[derive(Inspectable, Resource, Default)]
struct Data {
    #[inspectable(despawnable = true)]
    player: InspectorQuerySingle<Entity, With<Player>>,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(LogDiagnosticsPlugin::default())
                // .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(InspectorPlugin::<Data>::new())
                .add_system(bevy::window::close_on_esc);
        }
    }
}
