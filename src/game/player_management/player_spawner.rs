use crate::{prelude::*, read_no_field_variant};

use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub struct PlayerSpawnerPlugin;

impl Plugin for PlayerSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            respawn_player_on_game_restart.in_set(GameRestartSystemSet::Spawning),
        );
    }
}

fn respawn_player_on_game_restart(
    mut event_reader: EventReader<GameEvent>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    player_input_map: Res<PlayerInputMap>,
    commands: Commands,
) {
    for _restart_event in read_no_field_variant!(event_reader, GameEvent::RestartGame) {
        spawn_player(meshes, materials, player_input_map, commands);
        break;
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
        // TODO add StateScoped(AppState::Game),
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
        Player::default(),
        WorldBoundsWrapped,
        PlayerMonsterCollider::new(PLAYER_COLLIDER_RADIUS),
    ));
}
