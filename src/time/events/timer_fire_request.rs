use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy)]
pub struct TimerFireRequest {
    pub timer: EmittingTimer,
    pub parent_sequence: Option<TimerParentSequence>,
}

pub struct TimerFireRequestPlugin;

impl Plugin for TimerFireRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TimerFireRequest>();
    }
}
