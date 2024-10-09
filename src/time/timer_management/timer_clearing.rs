use crate::prelude::*;

pub struct TimerClearingPlugin;

impl Plugin for TimerClearingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            clear_done_timers.in_set(EndOfFrameSystemSet::TimerClearing),
        );
    }
}

pub fn clear_done_timers(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    emitting_timers: Query<(Entity, &EmittingTimer)>,
    mut commands: Commands,
) {
    for timer_done_event in timer_done_event_reader.read() {
        if let Ok((timer_entity, timer)) = emitting_timers.get(timer_done_event.timer_entity) {
            despawn_recursive_notify_on_fail(timer_entity, "EmittingTimer", &mut commands);
            if timer_done_event.timer_parent_sequence.is_none() {
                for value_calculator_entity in timer.calculator_entities_iter() {
                    despawn_recursive_notify_on_fail(
                        value_calculator_entity,
                        "an EmittingTimer's ValueCalculator",
                        &mut commands,
                    );
                }
            }
        }
    }
}
