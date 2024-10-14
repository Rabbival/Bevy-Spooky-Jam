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
    monsters_query: Query<(&Monster, &AffectingTimerCalculators)>,
    emitting_timer_with_parent_sequence_query: Query<(&EmittingTimer, &TimerParentSequence)>,
    mut commands: Commands,
) {
    for event in monster_state_set_listener.read() {
        if let MonsterState::Spawning = event.previous_state {
            continue;
        }
        if let MonsterState::Idle = event.next_state {
            match monsters_query.get(event.monster) {
                Ok((monster, affecting_timer_calculators)) => {
                    if cancel_stray_path_timer_and_begin_next_path_one(
                        &mut timer_done_event_writer,
                        &monster,
                        affecting_timer_calculators,
                        &emitting_timer_with_parent_sequence_query,
                        &mut commands,
                    )
                    .is_none()
                    {
                        print_error(
                            MonsterError::NoPathSequenceFoundOnStateChange(event.next_state),
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
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timer_with_parent_sequence_query: &Query<(&EmittingTimer, &TimerParentSequence)>,
    commands: &mut Commands,
) -> Option<TimerDoneEvent> {
    let maybe_timer_done_event = despawn_stray_path_timer_and_get_done_event(
        monster,
        affecting_timer_calculators,
        emitting_timer_with_parent_sequence_query,
        commands,
    );
    if let Some(timer_done_event) = maybe_timer_done_event {
        timer_done_event_writer.send(timer_done_event);
    }
    maybe_timer_done_event
}

fn despawn_stray_path_timer_and_get_done_event(
    monster: &Monster,
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timer_with_parent_sequence_query: &Query<(&EmittingTimer, &TimerParentSequence)>,
    commands: &mut Commands,
) -> Option<TimerDoneEvent> {
    if let Some(direct_line_movers) =
        affecting_timer_calculators.get(&TimerGoingEventType::Move(MovementType::InDirectLine))
    {
        for timer_and_calculator in direct_line_movers {
            let timer_entity = timer_and_calculator.timer;
            if let Ok((timer, parent_sequence)) =
                emitting_timer_with_parent_sequence_query.get(timer_and_calculator.timer)
            {
                if let Some(timer_sequence) = monster.path_timer_sequence {
                    if timer_sequence == parent_sequence.parent_sequence {
                        despawn_recursive_notify_on_fail(
                            timer_and_calculator.timer,
                            "timer when changing monster state",
                            commands,
                        );
                        return Some(TimerDoneEvent {
                            event_type: timer.send_once_done,
                            affected_entities: timer.affected_entities,
                            timer_entity,
                            timer_parent_sequence: Some(*parent_sequence),
                        });
                    }
                }
            }
        }
    }
    None
}
