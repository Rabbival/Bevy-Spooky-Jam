use crate::prelude::*;

pub mod calculate_and_send_going_event;
pub mod remove_from_timer_affected_entities;
pub mod set_time_multiplier;
pub mod timer_done_event;
pub mod timer_fire_request;
pub mod timer_going_event;
pub mod update_affected_entity_after_timer_birth;
pub mod value_calculator_event_channel;

pub struct TimeEventChannelPlugin;

impl Plugin for TimeEventChannelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SetTimeMultiplierPlugin,
            RemoveFromTimerAffectedEntitiesPlugin,
            CalculateAndSendGoingEventPlugin,
            TimerDoneEventPlugin,
            TimerFireRequestPlugin,
            UpdateAffectedEntitiesAfterTimerBirthPlugin,
            ValueCalculatorEventChannelPlugin,
        ));
    }
}
