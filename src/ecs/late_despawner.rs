use crate::prelude::*;

pub struct LateDespawnerPlugin;

impl Plugin for LateDespawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_despawn_requests_from_timers.in_set(EndOfFrameSystemSet::LateDespawn),
        );
    }
}

pub fn listen_for_despawn_requests_from_timers(
    mut event_reader: EventReader<TimerDoneEvent>,
    mut remove_from_timer_event_writer: EventWriter<RemoveFromTimerAffectedEntities>,
    affecting_timers_query: Query<&AffectingTimerCalculators>,
    timer_sequences_query: Query<&TimerSequence>,
    timer_query: Query<(Entity, &EmittingTimer, Option<&TimerParentSequence>)>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        if let TimerDoneEventType::DespawnAffectedEntities(despawn_policy) = event.event_type {
            for affected_entity in event.affected_entities.iter() {
                match despawn_policy {
                    DespawnPolicy::DespawnSelf => {}
                    DespawnPolicy::DespawnSelfAndRemoveFromAffectingTimers => {
                        remove_from_all_affecting_entities(
                            &mut remove_from_timer_event_writer,
                            &affecting_timers_query,
                            affected_entity.affected_entity,
                        );
                    }
                    DespawnPolicy::DespawnSelfAndAllThatAffectsIt => {
                        despawn_all_that_affect(
                            affected_entity.affected_entity,
                            &timer_sequences_query,
                            &timer_query,
                            &mut commands,
                        );
                    }
                }
                despawn_recursive_notify_on_fail(
                    affected_entity.affected_entity,
                    "(affected entity from timer despawn affected entities request)",
                    &mut commands,
                );
            }
        }
    }
}

fn remove_from_all_affecting_entities(
    remove_from_timer_event_writer: &mut EventWriter<RemoveFromTimerAffectedEntities>,
    affecting_timers_query: &Query<&AffectingTimerCalculators>,
    affected_entity: Entity,
) {
    if let Ok(affecting_timers) = affecting_timers_query.get(affected_entity) {
        for affecting_timers_of_type in affecting_timers.values() {
            for affecting_timer in affecting_timers_of_type {
                remove_from_timer_event_writer.send(RemoveFromTimerAffectedEntities {
                    timer_entity: affecting_timer.timer,
                    entity_to_remove: TimerAffectedEntity {
                        affected_entity,
                        value_calculator_entity: Some(affecting_timer.value_calculator),
                    },
                });
            }
        }
    } else {
        print_warning(
            format!(
                "Was asked to remove entity {:?} from affecting timers, but it has none.",
                affected_entity
            ),
            vec![LogCategory::RequestNotFulfilled],
        );
    }
}

fn despawn_all_that_affect(
    affected_entity: Entity,
    timer_sequences_query: &Query<&TimerSequence>,
    timer_query: &Query<(Entity, &EmittingTimer, Option<&TimerParentSequence>)>,
    commands: &mut Commands,
) {
    for (timer_entity, timer, maybe_parent) in timer_query {
        for timer_affected_entity in timer.affected_entities.iter() {
            if timer_affected_entity.affected_entity == affected_entity {
                if let Some(calculator) = timer_affected_entity.value_calculator_entity {
                    despawn_recursive_notify_on_fail(
                        calculator,
                        "calculator on late despawn",
                        commands,
                    );
                }
                despawn_recursive_notify_on_fail(timer_entity, "timer on late despawn", commands);
                if let Some(parent_sequence) = maybe_parent {
                    if timer_sequences_query
                        .get(parent_sequence.parent_sequence)
                        .is_ok()
                    {
                        despawn_recursive_notify_on_fail(
                            parent_sequence.parent_sequence,
                            "timer parent sequence on late despawn",
                            commands,
                        );
                    }
                }
            }
        }
    }
}
