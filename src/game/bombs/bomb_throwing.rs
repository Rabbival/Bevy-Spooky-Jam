use crate::{prelude::*, read_no_field_variant};

pub struct BombThrowingPlugin;

impl Plugin for BombThrowingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_bomb_text_color_after_throw,
                listen_for_bomb_throwing_requests,
            )
                .chain()
                .in_set(InputSystemSet::Handling),
        );
    }
}

fn listen_for_bomb_throwing_requests(
    mut player_request_listener: EventReader<PlayerRequest>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
    mut player_query: Query<(&mut Player, &Transform), Without<Bomb>>,
    mut bomb_query: Query<(&mut Bomb, &mut Transform), Without<Player>>,
    cursor_world_position: Res<CursorWorldPosition>,
    mut commands: Commands,
) {
    for _bomb_throw_request in
        read_no_field_variant!(player_request_listener, PlayerRequest::ThrowBomb)
    {
        for (mut player, player_transform) in &mut player_query {
            if let Some(bomb_entity) = player.held_bomb.take() {
                if let Ok((mut bomb, mut bomb_transform)) = bomb_query.get_mut(bomb_entity) {
                    disconnect_bomb_from_player(
                        player_transform,
                        bomb_entity,
                        &mut bomb_transform,
                        &mut bomb,
                        &mut commands,
                    );
                    fire_bomb_and_unslow_time(
                        &mut timer_fire_request_writer,
                        &mut time_multiplier_request_writer,
                        bomb_entity,
                        &bomb_transform,
                        &bomb,
                        cursor_world_position.0,
                        &mut commands,
                    );
                } else {
                    print_error(
                        EntityError::EntityNotInQuery(
                            "bomb entity when trying to shoot bomb, from the player's held bomb field",
                        ),
                        vec![LogCategory::RequestNotFulfilled, LogCategory::Player],
                    );
                }
            }
        }
    }
}

fn disconnect_bomb_from_player(
    player_transform: &Transform,
    bomb_entity: Entity,
    bomb_transform: &mut Transform,
    bomb: &mut Bomb,
    commands: &mut Commands,
) {
    bomb.bomb_state = BombState::Ticking;
    commands.entity(bomb_entity).remove::<Parent>();
    bomb_transform.translation += player_transform.translation; //now its transform is no longer relative to the player
}

fn fire_bomb_and_unslow_time(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    time_multiplier_request_writer: &mut EventWriter<SetTimeMultiplier>,
    bomb_entity: Entity,
    bomb_transform: &Transform,
    bomb: &Bomb,
    cursor_position: Vec2,
    commands: &mut Commands,
) {
    let throw_value_calculator = bomb_throw_calculator(bomb_transform, cursor_position, commands);
    let countdown_calculator = bomb_countdown_calculator(bomb, commands);
    timer_fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: bomb_entity,
                value_calculator_entity: Some(throw_value_calculator),
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            BOMB_THROW_TIME,
            TimerDoneEventType::Nothing,
        ),
        parent_sequence: None,
    });
    timer_fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: bomb_entity,
                value_calculator_entity: Some(throw_value_calculator),
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            BOMB_THROW_TIME,
            TimerDoneEventType::Nothing,
        ),
        parent_sequence: None,
    });
    timer_fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: bomb_entity,
                value_calculator_entity: Some(countdown_calculator),
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            bomb.full_duration as f32,
            TimerDoneEventType::ExplodeInRadius(BOMB_EXPLOSION_RADIUS),
        ),
        parent_sequence: None,
    });
    time_multiplier_request_writer.send(SetTimeMultiplier {
        multiplier_id: TimeMultiplierId::GameTimeMultiplier,
        new_multiplier: 1.0,
        duration: SLOW_MOTION_KICK_IN_AND_OUT_TIME,
    });
}

fn bomb_throw_calculator(
    bomb_transform: &Transform,
    cursor_position: Vec2,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::AppendToTimersOfType,
            ValueByInterpolation::from_goal_and_current(
                bomb_transform.translation,
                cursor_position.extend(Z_LAYER_BOMB),
                Interpolator::default(),
            ),
            TimerGoingEventType::Move(MovementType::InDirectLine),
        ))
        .id()
}

fn bomb_countdown_calculator(bomb: &Bomb, commands: &mut Commands) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::KeepNewTimer,
            ValueByInterpolation::from_goal_and_current(
                bomb.full_duration as f32,
                0.0,
                Interpolator::default(),
            ),
            TimerGoingEventType::BombCountdown,
        ))
        .id()
}

fn update_bomb_text_color_after_throw(
    mut player_request_listener: EventReader<PlayerRequest>,
    player_query: Query<&Player>,
    mut text_query: Query<(&mut Text, &Parent)>,
) {
    for _bomb_throw_request in
        read_no_field_variant!(player_request_listener, PlayerRequest::ThrowBomb)
    {
        for player in &player_query {
            if let Some(bomb_entity) = player.held_bomb {
                for (mut text, text_parent_entity) in &mut text_query {
                    if text_parent_entity.get() == bomb_entity {
                        text.sections[0].style.color = BombState::Ticking.to_color().text;
                    }
                }
            }
        }
    }
}
