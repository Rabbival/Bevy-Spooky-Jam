use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct UpdateAffectedEntitiesAfterTimerBirth {
    pub timer_entity: Entity,
    pub newborn_timer: EmittingTimer,
}

pub struct UpdateAffectedEntitiesAfterTimerBirthPlugin;

impl Plugin for UpdateAffectedEntitiesAfterTimerBirthPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdateAffectedEntitiesAfterTimerBirth>();
    }
}
