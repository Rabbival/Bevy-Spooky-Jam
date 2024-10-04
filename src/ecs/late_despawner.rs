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
                }
                despawn_entity_notify_on_fail(
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
