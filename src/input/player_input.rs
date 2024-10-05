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
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    player_query: Query<(&ActionState<PlayerAction>, &Transform, Entity)>,
    mut commands: Commands,
) {
    for (action_map, player_transform, player_entity) in &player_query {
        for action in action_map.get_pressed() {
            match action {
                PlayerAction::Move(move_direction) => {
                    send_player_movement_request(
                        &mut timer_fire_request_writer,
                        player_transform,
                        move_direction,
                        player_entity,
                        &mut commands,
                    );
                }
                _ => {}
            };
        }
    }
}

fn send_player_movement_request(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    player_transform: &Transform,
    move_direction: BasicDirection,
    player_entity: Entity,
    commands: &mut Commands,
) {
    let value_calculator = spawn_player_movement_calculator(
        player_transform,
        Vec3::from((
            PLAYER_MOVEMENT_DELTA * move_direction.to_world_direction(),
            0.0,
        )),
        commands,
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

fn listen_for_player_just_pressed_controls(
    mut player_query: Query<&ActionState<PlayerAction>, With<Player>>,
) {
    for action_map in &mut player_query {
        for action in action_map.get_just_pressed() {
            match action {
                PlayerAction::Fire => {
                    print_info("throwing a pumpkin bomb", vec![LogCategory::Player]);
                }
                _ => {}
            };
        }
    }
}
