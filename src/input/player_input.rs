use crate::prelude::*;

pub struct PlayerInputHandlerPlugin;

impl Plugin for PlayerInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                listen_for_player_pressed_controls,
                listen_for_player_just_pressed_controls,
                listen_for_player_just_released_controls,
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
        if let Some(normalized_movement_vector) = determine_move_direction(action_map) {
            player_request_writer.send(PlayerRequest::Move(normalized_movement_vector));
        }
    }
}

fn determine_move_direction(action_map: &ActionState<PlayerAction>) -> Option<Vec2> {
    let mut movement_direction = Vec2::ZERO;
    for action in action_map.get_pressed() {
        if action == PlayerAction::Move(BasicDirection::Up) {
            movement_direction.y += 1.0;
        }
        if action == PlayerAction::Move(BasicDirection::Down) {
            movement_direction.y -= 1.0;
        }
        if action == PlayerAction::Move(BasicDirection::Right) {
            movement_direction.x += 1.0;
        }
        if action == PlayerAction::Move(BasicDirection::Left) {
            movement_direction.x -= 1.0;
        }
    }
    movement_direction = movement_direction.normalize_or_zero();
    if movement_direction == Vec2::ZERO {
        None
    } else {
        Some(movement_direction)
    }
}

fn listen_for_player_just_pressed_controls(
    mut player_request_writer: EventWriter<PlayerRequest>,
    mut player_query: Query<(&ActionState<PlayerAction>, &Player)>,
) {
    for (action_map, player) in &mut player_query {
        for action in action_map.get_just_pressed() {
            match action {
                PlayerAction::BombInteraction => {
                    if player.held_bomb.is_none()
                        || FunctionalityOverride::PlayerMayCarryInfiniteBombs.enabled()
                    {
                        player_request_writer.send(PlayerRequest::PickUpBomb);
                    } else {
                        print_info(
                            "can't pick a bomb, the player already has one",
                            vec![LogCategory::Player],
                        );
                    }
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
