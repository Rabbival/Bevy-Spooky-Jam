use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct RemoveFromTimerAffectedEntities {
    pub timer_entity: Entity,
    pub entity_to_remove: TimerAffectedEntity,
}

pub struct RemoveFromTimerAffectedEntitiesPlugin;

impl Plugin for RemoveFromTimerAffectedEntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RemoveFromTimerAffectedEntities>();
    }
}
