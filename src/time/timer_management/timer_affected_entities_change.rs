use crate::prelude::*;

pub struct TimerAffectedEntitiesChangePlugin;

impl Plugin for TimerAffectedEntitiesChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_affected_entity_removal_request
                .in_set(EndOfFrameSystemSet::PreTimerClearing),
        );
    }
}

pub fn listen_for_affected_entity_removal_request(
    mut event_reader: EventReader<RemoveFromTimerAffectedEntities>,
    mut emitting_timers: Query<&mut EmittingTimer>,
) {
    for removal_request in event_reader.read() {
        if let Err(time_related_error) =
            remove_affected_entity(removal_request, &mut emitting_timers)
        {
            print_warning(time_related_error, vec![LogCategory::RequestNotFulfilled])
        } else {
            print_info(
                format!(
                    "Removed entity {:?} from timer: {:?}",
                    removal_request.entity_to_remove, removal_request.timer_entity
                ),
                vec![LogCategory::Time],
            );
        }
    }
}

fn remove_affected_entity(
    removal_request: &RemoveFromTimerAffectedEntities,
    emitting_timers: &mut Query<&mut EmittingTimer>,
) -> Result<(), TimeRelatedError> {
    let timer_entity = removal_request.timer_entity;
    let entity_to_remove = removal_request.entity_to_remove;
    if let Ok(mut emitting_timer) = emitting_timers.get_mut(timer_entity) {
        emitting_timer
            .affected_entities
            .remove_by_item(entity_to_remove)?;
        Ok(())
    } else {
        Err(TimeRelatedError::TimerToRemoveFromNotFound(
            *removal_request,
        ))
    }
}
