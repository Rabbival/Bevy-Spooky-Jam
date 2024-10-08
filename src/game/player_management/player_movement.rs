use crate::{prelude::*, read_single_field_variant};
use crate::game::player_management::consts::PLAYER_MOVEMENT_DELTA;

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
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    player_query: Query<(&Transform, Entity), With<Player>>,
    mut commands: Commands,
) {
    for normalized_move_direction in
        read_single_field_variant!(player_request_listener, PlayerRequest::Move)
    {
        for (player_transform, player_entity) in &player_query {
            let value_calculator = spawn_player_movement_calculator(
                player_transform,
                Vec3::from((PLAYER_MOVEMENT_DELTA * *normalized_move_direction, 0.0)),
                &mut commands,
            );
            timer_fire_request_writer.send(TimerFireRequest {
                timer: EmittingTimer::new(
                    vec![TimerAffectedEntity {
                        affected_entity: player_entity,
                        value_calculator_entity: Some(value_calculator),
                    }],
                    vec![TimeMultiplierId::GameTimeMultiplier],
                    0.01,
                    TimerDoneEventType::Nothing,
                ),
                parent_sequence: None,
            });
        }
    }
}

fn spawn_player_movement_calculator(
    player_transform: &Transform,
    delta: Vec3,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::KeepNewTimer,
            ValueByInterpolation::new(player_transform.translation, delta, Interpolator::new(1.2)),
            TimerGoingEventType::Move(MovementType::InDirectLine),
        ))
        .id()
}
