use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use leafwing_input_manager::prelude::*;

use crate::input::PlayerAction;
use crate::prelude::InputSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, handle_player_controls.in_set(InputSystemSet::Handling))
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
        (PlayerAction::MoveUp, KeyCode::KeyW),
        (PlayerAction::MoveRight, KeyCode::KeyD),
        (PlayerAction::MoveDown, KeyCode::KeyS),
        (PlayerAction::Fire, KeyCode::Enter),
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
    ));
}

fn handle_player_controls(query: Query<&ActionState<PlayerAction>, With<Player>>) {
    let action_state = query.single();
    // Each action has a button-like state of its own that you can check
    if action_state.just_pressed(&PlayerAction::MoveLeft) {
        info!("I'm moving left!");
    }
    if action_state.just_pressed(&PlayerAction::MoveUp) {
        info!("I'm moving up!");
    }
    if action_state.just_pressed(&PlayerAction::MoveRight) {
        info!("I'm moving right!");
    }
    if action_state.just_pressed(&PlayerAction::MoveDown) {
        info!("I'm moving down!");
    }
    if action_state.just_pressed(&PlayerAction::Fire) {
        info!("I'm throwing a pumpkin bomb!");
    }
}

// Components
#[derive(Component)]
pub struct Player;
