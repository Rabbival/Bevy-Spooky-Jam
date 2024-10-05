use crate::prelude::*;

pub struct PlayerInputHandlerPlugin;

impl Plugin for PlayerInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_player_controls, handle_player_just_pressed_controls)
                .in_set(InputSystemSet::Handling),
        );
    }
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
                print_info("throwing a pumpkin bomb", vec![LogCategory::Player]);
            }
        }
    }
}
