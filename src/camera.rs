use super::player::Player;
use crate::AppState;
use bevy::{input::mouse::MouseWheel, prelude::*};

const Z_CAM: f32 = 100.;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(load_camera))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(camera_follow_player)
                    .with_system(zoom_camera),
            );
    }
}

fn load_camera(mut commands: Commands) {
    let _camera_entity = commands
        .spawn_bundle(Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, Z_CAM),
            projection: OrthographicProjection {
                scale: 0.5,
                ..default()
            },
            ..default()
        })
        .id();
}

fn camera_follow_player(
    mut camera: Query<(&mut Transform, &Camera2d), Without<Player>>,
    players: Query<(&mut Transform, &Player), Without<Camera2d>>,
) {
    for (player, _) in players.iter() {
        for (mut cam, _) in camera.iter_mut() {
            cam.translation.x = player.translation.x;
            cam.translation.y = player.translation.y;
        }
    }
}

//TODO: zoom_scroll_speed could become a component probably?
//TODO: could try to abstract input from this function
fn zoom_camera(
    mut camera_query: Query<(&mut Transform, &Camera2d), Without<Player>>,
    mut scroll_wheel: EventReader<MouseWheel>,
) {
    let (mut cam, _) = camera_query.single_mut();
    let zoom_scroll_speed = 0.05;
    for direction in scroll_wheel.iter() {
        cam.scale = (cam.scale + zoom_scroll_speed * direction.y)
            .clamp(Vec3::new(0.2, 0.2, 0.2), Vec3::new(2.0, 2.0, 2.0));
    }
}
