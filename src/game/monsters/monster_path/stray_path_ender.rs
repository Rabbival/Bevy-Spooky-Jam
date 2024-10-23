use crate::prelude::*;

pub struct MonsterStrayPathEnderPlugin;

impl Plugin for MonsterStrayPathEnderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            end_stray_path_when_back_to_idle.in_set(MonsterSystemSet::PathAndVisualUpdating),
        );
    }
}

fn end_stray_path_when_back_to_idle(
    mut monster_state_set_listener: EventReader<MonsterStateChanged>,
    mut timer_done_event_writer: EventWriter<TimerDoneEvent>,
    mut monsters_query: Query<(&Monster, &mut AffectingTimerCalculators)>,
    emitting_timer_with_parent_sequence_query: Query<(&EmittingTimer, &TimerParentSequence)>,
    mut commands: Commands,
) {
    for event in monster_state_set_listener.read() {
        if let MonsterState::Spawning = event.previous_state {
            continue;
        }
        if let MonsterState::Idle = event.next_state {
            match monsters_query.get_mut(event.monster) {
                Ok((monster, mut affecting_timer_calculators)) => {
                    if let Err(monster_error) = cancel_stray_path_timer_and_begin_next_path_one(
                        &mut timer_done_event_writer,
                        monster,
                        &mut affecting_timer_calculators,
                        &emitting_timer_with_parent_sequence_query,
                        &mut commands,
                    ) {
                        print_error(
                            format!(
                                "{}, cause by: {}",
                                MonsterError::NoPathSequenceFoundOnStateChange(event.next_state),
                                monster_error
                            ),
                            vec![LogCategory::RequestNotFulfilled, LogCategory::Monster],
                        );
                    }
                }
                Err(_) => {
                    print_error(
                        EntityError::EntityNotInQuery("monster when asked to initiate idle state"),
                        vec![LogCategory::RequestNotFulfilled, LogCategory::Monster],
                    );
                }
            }
        }
    }
}

fn cancel_stray_path_timer_and_begin_next_path_one(
    timer_done_event_writer: &mut EventWriter<TimerDoneEvent>,
    monster: &Monster,
    affecting_timer_calculators: &mut AffectingTimerCalculators,
    emitting_timer_with_parent_sequence_query: &Query<(&EmittingTimer, &TimerParentSequence)>,
    commands: &mut Commands,
) -> Result<(), MonsterError> {
    let timer_done_event = despawn_stray_path_timer_and_get_done_event(
        monster,
        affecting_timer_calculators,
        emitting_timer_with_parent_sequence_query,
        commands,
    )?;
    timer_done_event_writer.send(timer_done_event);
    Ok(())
}

fn despawn_stray_path_timer_and_get_done_event(
    monster: &Monster,
    affecting_timer_calculators: &mut AffectingTimerCalculators,
    emitting_timer_with_parent_sequence_query: &Query<(&EmittingTimer, &TimerParentSequence)>,
    commands: &mut Commands,
) -> Result<TimerDoneEvent, MonsterError> {
    let direct_line_mover_type = &TimerGoingEventType::Move(MovementType::InDirectLine);
    match affecting_timer_calculators.get(direct_line_mover_type) {
        Some(direct_line_movers) => match monster.path_timer_sequence {
            Some(monster_current_path_sequence) => {
                for timer_and_calculator in direct_line_movers {
                    let timer_entity = timer_and_calculator.timer;
                    if let Ok((timer, parent_sequence)) =
                        emitting_timer_with_parent_sequence_query.get(timer_and_calculator.timer)
                    {
                        if monster_current_path_sequence == parent_sequence.parent_sequence {
                            despawn_recursive_notify_on_fail(
                                timer_and_calculator.timer,
                                "timer when changing monster state",
                                commands,
                            );
                            despawn_recursive_notify_on_fail(
                                timer_and_calculator.value_calculator,
                                "calculator when changing monster state",
                                commands,
                            );
                            affecting_timer_calculators
                                .remove(direct_line_mover_type, timer_and_calculator.timer);
                            return Ok(TimerDoneEvent {
                                event_type: timer.send_once_done,
                                affected_entities: timer.affected_entities,
                                timer_entity,
                                timer_parent_sequence: Some(*parent_sequence),
                            });
                        }
                        print_info("Found a movement timer with a parent sequence for monster, but it wasn't the one listed as path sequence in its struct", vec![LogCategory::Monster]);
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
