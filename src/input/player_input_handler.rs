use crate::prelude::*;

pub struct PlayerInputHandlerPlugin;

impl Plugin for PlayerInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                listen_for_just_pressed_player_input,
                listen_for_just_released,
            ),
        );
    }
}

fn listen_for_just_pressed_player_input(
    player_action_states: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    for action_state in &player_action_states {
        for just_pressed_action in action_state.get_just_pressed() {
            match just_pressed_action {
                PlayerAction::Move(move_direction) => {
                    print_info(
                        &format!("player asked to move in direction {:?}", move_direction),
                        vec![LogCategory::Player],
                    );
                }
                PlayerAction::PickUpOrReleaseBomb => {
                    print_info("player asked to pick up bomb", vec![LogCategory::Player]);
                }
            }
        }
    }
}

fn listen_for_just_released(player_action_states: Query<&ActionState<PlayerAction>, With<Player>>) {
    for action_state in &player_action_states {
        for just_released_action in action_state.get_just_released() {
            match just_released_action {
                PlayerAction::PickUpOrReleaseBomb => {
                    print_info("player asked to release bomb", vec![LogCategory::Player]);
                }
                _ => {}
            }
        }
    }
}
