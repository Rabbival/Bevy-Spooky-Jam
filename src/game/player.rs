use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use leafwing_input_manager::prelude::*;

use crate::input::PlayerAction;
use crate::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (handle_player_controls, handle_player_just_pressed_controls)
                .in_set(InputSystemSet::Handling),
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let input_map = InputMap::new([
        (PlayerAction::Move(BasicDirection::Left), KeyCode::KeyA),
        (PlayerAction::Move(BasicDirection::Left), KeyCode::ArrowLeft),
        (PlayerAction::Move(BasicDirection::Up), KeyCode::KeyW),
        (PlayerAction::Move(BasicDirection::Up), KeyCode::ArrowUp),
        (PlayerAction::Move(BasicDirection::Right), KeyCode::KeyD),
        (
            PlayerAction::Move(BasicDirection::Right),
            KeyCode::ArrowRight,
        ),
        (PlayerAction::Move(BasicDirection::Down), KeyCode::KeyS),
        (PlayerAction::Move(BasicDirection::Down), KeyCode::ArrowDown),
        (PlayerAction::Fire, KeyCode::Enter),
        (PlayerAction::Fire, KeyCode::NumpadEnter),
    ]);
    commands.spawn((
        // TODO StateScoped(AppState::Game),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Capsule2d::new(10.0, 20.0))),
            material: materials.add(Color::srgb(0.3, 0.9, 0.3)),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
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
    mut player_query: Query<(&ActionState<PlayerAction>, &mut Transform), With<Player>>,
) {
    for (action_map, mut player_transform) in &mut player_query {
        for action in action_map.get_pressed() {
            let delta = match action {
                PlayerAction::Move(move_direction) => 200.0 * move_direction.to_world_direction(),
                _ => Vec2::ZERO,
            };
            player_transform.translation += Vec3::new(delta.x, delta.y, 0.0) * time.delta_seconds()
        }
    }
}

fn handle_player_just_pressed_controls(
    mut player_query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    for action_map in &mut player_query {
        for action in action_map.get_just_pressed() {
            if let PlayerAction::Fire = action {
                print_info("throwing a bomb", vec![LogCategory::Player]);
            }
        }
    }
}
