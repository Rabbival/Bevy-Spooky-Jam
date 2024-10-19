use crate::{prelude::*, read_single_field_variant};

pub struct PlayerMovementPlugin;

impl Plugin for PlayerMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_player_movement_requests.in_set(InputSystemSet::Handling),
        );
    }
}

fn listen_for_player_movement_requests(
    mut player_request_listener: EventReader<PlayerRequest>,
    mut player_query: Query<(&mut Transform, Option<&WorldBoundsWrapped>), With<Player>>,
    time_multipliers: Query<&TimeMultiplier>,
    time: Res<Time>,
) {
    for normalized_move_direction in
        read_single_field_variant!(player_request_listener, PlayerRequest::Move)
    {
        for (mut player_transform, maybe_bounds_wrapped) in &mut player_query {
            for time_multiplier in &time_multipliers {
                if let TimeMultiplierId::GameTimeMultiplier = time_multiplier.id() {
                    let multiplied_time_delta = time.delta_seconds() * time_multiplier.value();
                    let move_direction_by_time =
                        (multiplied_time_delta * *normalized_move_direction).extend(0.0);
                    player_transform.translation += move_direction_by_time * PLAYER_SPEED;
                    if maybe_bounds_wrapped.is_some() {
                        player_transform.translation =
                            calculate_reach_beyond_screen_border(player_transform.translation);
                    }
                }
            }
        }
    }
}
