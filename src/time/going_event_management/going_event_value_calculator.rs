use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy)]
pub struct GoingEventValueCalculator<T: Numeric> {
    pub set_policy: TimerCalculatorSetPolicy,
    calculator: ValueByInterpolation<T>,
    going_event_type: TimerGoingEventType,
}

impl<T: Numeric> GoingEventValueCalculator<T> {
    pub fn new(
        set_policy: TimerCalculatorSetPolicy,
        calculator: ValueByInterpolation<T>,
        going_event_type: TimerGoingEventType,
    ) -> Self {
        Self {
            set_policy,
            calculator,
            going_event_type,
        }
    }

    pub fn get_full_delta(&self) -> T {
        self.calculator.delta()
    }

    pub fn get_timer_going_event(
        &mut self,
        normalized_progress: f32,
        affect_entity: Entity,
    ) -> TimerGoingEvent<T> {
        let value_delta = self.calculator.calculate_delta(normalized_progress);
        TimerGoingEvent {
            event_type: self.going_event_type,
            entity: affect_entity,
            value_delta,
        }
    }

    pub fn going_event_type(&self) -> TimerGoingEventType {
        self.going_event_type
    }

    pub fn initialize_calculator(&mut self) {
        self.calculator.initialize_previous_value();
    }
}
