use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct CalculateAndSendGoingEvent {
    pub going_event_value_calculator: Entity,
    pub affected_entity: Entity,
    pub normalized_progress: f32,
}

pub struct CalculateAndSendGoingEventPlugin;

impl Plugin for CalculateAndSendGoingEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CalculateAndSendGoingEvent>();
    }
}
