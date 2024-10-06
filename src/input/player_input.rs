use crate::prelude::*;

pub struct PlayerInputHandlerPlugin;

impl Plugin for PlayerInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                listen_for_player_pressed_controls,
                listen_for_player_just_pressed_controls,
            )
                .in_set(InputSystemSet::Listening),
        );
    }
}

fn listen_for_player_pressed_controls(
    mut player_request_writer: EventWriter<PlayerRequest>,
    player_query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    for action_map in &player_query {
        for action in action_map.get_pressed() {
            match action {
                PlayerAction::Move(move_direction) => {
                    player_request_writer.send(PlayerRequest::Move(move_direction));
                }
                _ => {}
            };
        }
    }
}

fn listen_for_player_just_pressed_controls(
    mut player_request_writer: EventWriter<PlayerRequest>,
    mut player_query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    for action_map in &mut player_query {
        for action in action_map.get_just_pressed() {
            match action {
                PlayerAction::BombInteraction => {
                    player_request_writer.send(PlayerRequest::PickUpBomb);
                }
                _ => {}
            };
        }
    }
}

fn listen_for_player_just_released_controls(
    mut player_request_writer: EventWriter<PlayerRequest>,
    mut player_query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    for action_map in &mut player_query {
        for action in action_map.get_just_released() {
            match action {
                PlayerAction::BombInteraction => {
                    player_request_writer.send(PlayerRequest::ThrowBomb);
                }
                _ => {}
            };
        }
    }
}
