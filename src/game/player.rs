use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use leafwing_input_manager::prelude::*;

use crate::input::PlayerAction;
use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(FixedUpdate, handle_player_controls.in_set(InputSystemSet::Handling))
        ;
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let input_map = InputMap::new([
        (PlayerAction::MoveLeft, KeyCode::KeyA),
        (PlayerAction::MoveLeft, KeyCode::ArrowLeft),
        (PlayerAction::MoveUp, KeyCode::KeyW),
        (PlayerAction::MoveUp, KeyCode::ArrowUp),
        (PlayerAction::MoveRight, KeyCode::KeyD),
        (PlayerAction::MoveRight, KeyCode::ArrowRight),
        (PlayerAction::MoveDown, KeyCode::KeyS),
        (PlayerAction::MoveDown, KeyCode::ArrowDown),
        (PlayerAction::Fire, KeyCode::Enter),
        (PlayerAction::Fire, KeyCode::NumpadEnter),
    ]);
    commands.spawn((
        // TODO StateScoped(AppState::Game),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Capsule2d::new(10.0, 20.0))),
            material: materials.add(Color::srgb(0.3, 0.9, 0.3)),
            transform: Transform::from_xyz(
                0.0,
                0.0,
                10.0,
            ),
            ..default()
        },
        InputManagerBundle::<PlayerAction> {
            action_state: ActionState::default(),
            input_map,
        },
        Player,
        WorldBoundsWrapped,
    ));
}

fn handle_player_controls(
    time: Res<Time>,
    action_state_query: Query<&ActionState<PlayerAction>, With<Player>>,
    mut player_transform_query: Query<&mut Transform, With<Player>>,
) {
    let action_state = action_state_query.single();
    let mut player_transform = player_transform_query.single_mut();
    // Each action has a button-like state of its own that you can check
    if action_state.pressed(&PlayerAction::MoveLeft) {
        player_transform.translation.x -= 200.0 * time.delta_seconds();
    }
    if action_state.pressed(&PlayerAction::MoveUp) {
        player_transform.translation.y += 200.0 * time.delta_seconds();
    }
    if action_state.pressed(&PlayerAction::MoveRight) {
        player_transform.translation.x += 200.0 * time.delta_seconds();
    }
    if action_state.pressed(&PlayerAction::MoveDown) {
        player_transform.translation.y -= 200.0 * time.delta_seconds();
    }
    if action_state.just_pressed(&PlayerAction::Fire) {
        info!("I'm throwing a pumpkin bomb!");
    }
}
