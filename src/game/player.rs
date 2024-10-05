use crate::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player_input_map: Res<PlayerInputMap>,
    mut commands: Commands,
) {
    let input_map = player_input_map.0.clone();
    commands.spawn((
        // TODO StateScoped(AppState::Game),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Capsule2d::new(10.0, 20.0))),
            material: materials.add(Color::srgb(0.3, 0.9, 0.3)),
            transform: Transform::from_xyz(0.0, 0.0, Z_LAYER_PLAYER),
            ..default()
        },
        InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        },
        AffectingTimerCalculators::default(),
        Player,
        WorldBoundsWrapped,
    ));
}
