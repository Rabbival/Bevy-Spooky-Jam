use crate::prelude::*;

pub mod affecting_timer_calculators_management;
pub mod consts;
pub mod emitting_timer;
pub mod errors;
pub mod events;
pub mod going_event_management;
pub mod time_multiplication;
pub mod timer_affected_entity;
pub mod timer_and_calculator;
pub mod timer_management;
pub mod timer_sequencing;

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TimerManagementPlugin,
            TimeMutiplicationPlugin,
            TimeEventChannelPlugin,
            TimerSequenceManagerPlugin,
        ));
    }
}
