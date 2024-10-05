use crate::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

pub fn spawn_player(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_input_map: Res<PlayerInputMap>,
    mut commands: Commands,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Circle::new(ORB_MAX_RADIUS))),
            material: materials.add(Color::srgb(0.0, 0.8, 0.6)),
            transform: Transform::from_xyz(0.0, -50.0, 0.0),
            ..default()
        },
        AffectingTimerCalculators::default(),
        InputManagerBundle::with_map(player_input_map.0.clone()),
        Player,
    ));
}
