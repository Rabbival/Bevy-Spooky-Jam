use crate::prelude::*;

pub struct MonsterStrayPathUpdaterPlugin;

impl Plugin for MonsterStrayPathUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (listen_for_state_set, update_stray_path)
                .in_set(MonsterSystemSet::PathAndVisualUpdating),
        );
    }
}

fn listen_for_state_set(
    mut monster_state_set_listener: EventReader<MonsterStateChanged>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut new_path_event_writer: EventWriter<MonsterStrayPathUpdated>,
    mut monsters_query: Query<(Entity, &Monster, &Transform, &mut AffectingTimerCalculators)>,
    transforms: Query<&Transform>,
    emitting_timer_parent_sequence_query: Query<&TimerParentSequence, With<EmittingTimer>>,
    mut commands: Commands,
) {
    for event in monster_state_set_listener.read() {
        match monsters_query.get_mut(event.monster) {
            Ok((monster_entity, monster, monster_transform, mut affecting_timer_calculators)) => {
                if let MonsterState::Chasing(target_entity) | MonsterState::Fleeing(target_entity) =
                    event.next_state
                {
                    if let Ok(target_transform) = transforms.get(target_entity) {
                        match replace_current_path_get_new_delta(
                            target_transform.translation,
                            monster,
                            monster_entity,
                            monster_transform.translation,
                            &mut affecting_timer_calculators,
                            &emitting_timer_parent_sequence_query,
                            &mut timer_fire_request_writer,
                            &mut commands,
                            event.previous_state,
                        ) {
                            Ok(new_delta) => {
                                new_path_event_writer.send(MonsterStrayPathUpdated {
                                    new_delta,
                                    monster_entity,
                                });
                            }
                            Err(monster_error) => {
                                print_error(
                                    monster_error,
                                    vec![LogCategory::RequestNotFulfilled, LogCategory::Monster],
                                );
                            }
                        }
                    }
                }
            }
            Err(_) => {
                print_warning(
                    EntityError::EntityNotInQuery(
                        "monster after state change when trying to set path",
                    ),
                    vec![LogCategory::RequestNotFulfilled, LogCategory::Monster],
                );
            }
        }
    }
}

fn update_stray_path(
    monster_state_set_listener: EventReader<MonsterStateChanged>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut new_path_event_writer: EventWriter<MonsterStrayPathUpdated>,
    just_changed_transforms: Query<&Transform, Changed<Transform>>,
    mut monsters_query: Query<(Entity, &Monster, &Transform, &mut AffectingTimerCalculators)>,
    emitting_timer_parent_sequence_query: Query<&TimerParentSequence, With<EmittingTimer>>,
    mut commands: Commands,
) {
    if monster_state_set_listener.len() > 0 {
        return;
    }
    for (monster_entity, monster, monster_transform, mut affecting_timer_calculators) in
        &mut monsters_query
    {
        if let MonsterState::Chasing(target_entity) | MonsterState::Fleeing(target_entity) =
            monster.state
        {
            if let Ok(target_transform) = just_changed_transforms.get(target_entity) {
                match replace_current_path_get_new_delta(
                    target_transform.translation,
                    monster,
                    monster_entity,
                    monster_transform.translation,
                    &mut affecting_timer_calculators,
                    &emitting_timer_parent_sequence_query,
                    &mut timer_fire_request_writer,
                    &mut commands,
                    monster.state,
                ) {
                    Ok(new_delta) => {
                        new_path_event_writer.send(MonsterStrayPathUpdated {
                            new_delta,
                            monster_entity,
                        });
                    }
                    Err(monster_error) => {
                        print_error(
                            monster_error,
                            vec![LogCategory::RequestNotFulfilled, LogCategory::Monster],
                        );
                    }
                }
            }
        }
    }
}

fn replace_current_path_get_new_delta(
    target_location: Vec3,
    monster: &Monster,
    monster_entity: Entity,
    monster_location: Vec3,
    affecting_timer_calculators: &mut AffectingTimerCalculators,
    emitting_timer_parent_sequence_query: &Query<&TimerParentSequence, With<EmittingTimer>>,
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    commands: &mut Commands,
    monster_previous_state: MonsterState,
) -> Result<Vec2, MonsterError> {
    let should_destroy_previous_calculator =
        if let MonsterState::Chasing(_) | MonsterState::Fleeing(_) = monster_previous_state {
            true
        } else {
            false
        };
    let speed = if let MonsterState::Chasing(_) = monster.state {
        MONSTER_SPEED_WHEN_CHASING
    } else {
        MONSTER_SPEED_WHEN_FLEEING
    };
    let location_to_move_towards = if let MonsterState::Chasing(_) = monster.state {
        target_location
    } else {
        target_location
            + (monster_location - target_location).normalize() * BOMB_EXPLOSION_RADIUS * 1.65
    };
    let timer_duration = location_to_move_towards.distance(monster_location) / speed;
    let new_path_calculator =
        spawn_monster_move_calculator(monster_location, location_to_move_towards, commands);
    let path_timer_parent_sequence = destroy_current_path_timer_and_calculator(
        monster,
        affecting_timer_calculators,
        emitting_timer_parent_sequence_query,
        commands,
        should_destroy_previous_calculator,
    )?;
    timer_fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: monster_entity,
                value_calculator_entity: Some(new_path_calculator),
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            timer_duration,
            TimerDoneEventType::SetAnimationCycleByPathParentSequence,
        ),
        parent_sequence: Some(path_timer_parent_sequence),
    });
    timer_fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: monster_entity,
                value_calculator_entity: None,
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            timer_duration,
            TimerDoneEventType::UpdateState,
        ),
        parent_sequence: None,
    });
    Ok((location_to_move_towards - monster_location).truncate())
}

fn destroy_current_path_timer_and_calculator(
    monster: &Monster,
    affecting_timer_calculators: &mut AffectingTimerCalculators,
    emitting_timer_parent_sequence_query: &Query<&TimerParentSequence, With<EmittingTimer>>,
    commands: &mut Commands,
    destroy_calculator: bool,
) -> Result<TimerParentSequence, MonsterError> {
    let direct_line_mover_type = &TimerGoingEventType::Move(MovementType::InDirectLine);
    match affecting_timer_calculators.get(direct_line_mover_type) {
        Some(direct_line_movers) => match monster.path_timer_sequence {
            Some(monster_assigned_path_sequence) => {
                for timer_and_calculator in direct_line_movers {
                    if let Ok(parent_sequence) =
                        emitting_timer_parent_sequence_query.get(timer_and_calculator.timer)
                    {
                        if monster_assigned_path_sequence == parent_sequence.parent_sequence {
                            if destroy_calculator {
                                despawn_recursive_notify_on_fail(
                                    timer_and_calculator.value_calculator,
                                    "timer calculator when changing monster state",
                                    commands,
                                );
                            }
                            despawn_recursive_notify_on_fail(
                                timer_and_calculator.timer,
                                "timer when changing monster state",
                                commands,
                            );
                            affecting_timer_calculators
                                .remove(direct_line_mover_type, timer_and_calculator.timer);
                            return Ok(*parent_sequence);
                        }
                    }
                }
                if direct_line_movers.iter().count() > 0 {
                    Err(MonsterError::NoMovementTimerHadTheListedPathParentSequence)
                } else {
                    Err(MonsterError::NoMovementAffectingTimerFound)
                }
            }
            None => Err(MonsterError::MonsterHasNoPathTimerSequenceAssigned),
        },
        None => Err(MonsterError::NoMovementAffectingTimerFound),
    }
}

fn spawn_monster_move_calculator(
    current_location: Vec3,
    location_to_move_towards: Vec3,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::AppendToTimersOfType,
            ValueByInterpolation::from_goal_and_current(
                current_location,
                location_to_move_towards.with_z(Z_LAYER_MONSTER),
                Interpolator::new(1.0),
            ),
            TimerGoingEventType::Move(MovementType::InDirectLine),
        ))
        .id()
}
