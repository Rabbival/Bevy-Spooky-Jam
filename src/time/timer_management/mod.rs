use crate::prelude::*;

pub mod timer_affected_entities_change;
pub mod timer_clearing;
pub mod timer_firing;
pub mod timer_ticking;

pub struct TimerManagementPlugin;

impl Plugin for TimerManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TimerFiringPlugin,
            TimerTickingPlugin,
            TimerClearingPlugin,
            TimerAffectedEntitiesChangePlugin,
        ));
    }
}
