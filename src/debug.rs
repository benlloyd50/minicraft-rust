use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_inspector_egui::{widgets::InspectorQuerySingle, Inspectable, InspectorPlugin};

use crate::player::Player;

pub struct DebugPlugin;

#[derive(Inspectable, Default)]
struct Data {
    //Example queries for stuffs
    // query: InspectorQuery<Entity, With<Transform>>,
    // has_material: InspectorQuery<&'static mut Handle<StandardMaterial>>,
    #[inspectable(despawnable = true)]
    player: InspectorQuerySingle<Entity, With<Player>>,
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(LogDiagnosticsPlugin::default())
                .add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(InspectorPlugin::<Data>::new())
                //Dont want to close the game like this on release, so make sure debug is not included
                .add_system(bevy::window::close_on_esc);
        }
    }
}
