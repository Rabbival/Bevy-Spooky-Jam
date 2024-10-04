use crate::{plugin_for_implementors_of_trait, prelude::*};

plugin_for_implementors_of_trait!(GoingEventEmittingPlugin, Numeric);

impl<T: Numeric> Plugin for GoingEventEmittingPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            calculate_value_and_send_going_event::<T>
                .in_set(TickingSystemSet::PostTickingImmidiate),
        );
    }
}

pub fn calculate_value_and_send_going_event<T: Numeric>(
    mut calculation_requests_reader: EventReader<CalculateAndSendGoingEvent>,
    mut timer_going_event_writer: EventWriter<TimerGoingEvent<T>>,
    mut value_calculators: Query<&mut GoingEventValueCalculator<T>>,
) {
    for calculation_request in calculation_requests_reader.read() {
        if let Ok(mut value_calculator) =
            value_calculators.get_mut(calculation_request.going_event_value_calculator)
        {
            let timer_going_event = value_calculator.get_timer_going_event(
                calculation_request.normalized_progress,
                calculation_request.affected_entity,
            );
            timer_going_event_writer.send(timer_going_event);
        }
    }
}
