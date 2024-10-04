use crate::{plugin_for_implementors_of_trait, prelude::*};

plugin_for_implementors_of_trait!(AffectingTimerCalculatorsPlugin, Numeric);

impl<T: Numeric> Plugin for AffectingTimerCalculatorsPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            clear_done_timers_from_affecting_timers::<T>.in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn clear_done_timers_from_affecting_timers<T: Numeric>(
    mut emitting_timer_done_event_reader: EventReader<TimerDoneEvent>,
    mut timer_affected_entities: Query<&mut AffectingTimerCalculators>,
    calculator_entities: Query<&GoingEventValueCalculator<T>>,
) {
    for done_event in emitting_timer_done_event_reader.read() {
        for calculator_entity in done_event.affected_entities.calculator_entities_iter() {
            if let Ok(value_calculator) = calculator_entities.get(calculator_entity) {
                for mut affecting_timers in &mut timer_affected_entities {
                    affecting_timers.remove(
                        &value_calculator.going_event_type(),
                        done_event.timer_entity,
                    );
                }
            }
        }
    }
}
