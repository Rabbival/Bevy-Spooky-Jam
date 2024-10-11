use crate::prelude::*;
use bevy::render::view::RenderLayers;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2dBundle {
            camera: Camera::default(),
            transform: Transform::from_xyz(0.0, TOP_UI_HEADER_BAR_HEIGHT / 2.0, 1000.0),
            ..default()
        },
        RenderLayers::layer(0),
    ));
}
