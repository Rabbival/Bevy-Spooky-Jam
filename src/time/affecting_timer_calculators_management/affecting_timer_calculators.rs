use crate::prelude::*;

#[derive(Debug, Component, Clone, Default)]
pub struct AffectingTimerCalculators(HashMap<TimerGoingEventType, Vec<TimerAndCalculator>>);

impl AffectingTimerCalculators {
    pub fn get(&self, going_event_type: &TimerGoingEventType) -> Option<&Vec<TimerAndCalculator>> {
        self.0.get(going_event_type)
    }

    pub fn insert_get_rejected_value(
        &mut self,
        key: TimerGoingEventType,
        value: TimerAndCalculator,
        policy: TimerCalculatorSetPolicy,
    ) -> Option<Vec<TimerAndCalculator>> {
        match policy {
            TimerCalculatorSetPolicy::KeepNewTimer => self.0.insert(key, vec![value]),
            TimerCalculatorSetPolicy::IgnoreNewIfAssigned => {
                self.insert_only_if_theres_no_timer_of_that_type(key, value)
            }
            TimerCalculatorSetPolicy::AppendToTimersOfType => self.push_to_timers_vec(key, value),
        }
    }

    fn insert_only_if_theres_no_timer_of_that_type(
        &mut self,
        key: TimerGoingEventType,
        value: TimerAndCalculator,
    ) -> Option<Vec<TimerAndCalculator>> {
        let maybe_timers_of_that_type = self.get(&key);
        match maybe_timers_of_that_type {
            Some(timers_of_that_type) => {
                let owned_timers_of_that_type = timers_of_that_type.clone();
                if owned_timers_of_that_type.is_empty() {
                    self.0.insert(key, vec![value]);
                    None
                } else {
                    Some(vec![value])
                }
            }
            None => {
                self.0.insert(key, vec![value]);
                None
            }
        }
    }

    fn push_to_timers_vec(
        &mut self,
        key: TimerGoingEventType,
        value: TimerAndCalculator,
    ) -> Option<Vec<TimerAndCalculator>> {
        let maybe_timers_of_that_type = self.0.get_mut(&key);
        if let Some(timers_of_that_type) = maybe_timers_of_that_type {
            timers_of_that_type.push(value);
        } else {
            self.0.insert(key, vec![value]);
        }
        None
    }

    pub fn remove(
        &mut self,
        timer_type: &TimerGoingEventType,
        timer_entity: Entity,
    ) -> Option<TimerAndCalculator> {
        let maybe_timers_of_type = self.0.get_mut(timer_type);
        if let Some(timers_of_type) = maybe_timers_of_type {
            for (index, timer_and_calculator) in timers_of_type.iter().copied().enumerate() {
                if timer_and_calculator.timer == timer_entity {
                    timers_of_type.remove(index);
                    return Some(timer_and_calculator);
                }
            }
        }
        None
    }

    pub fn values(&self) -> impl Iterator<Item = &Vec<TimerAndCalculator>> + '_ {
        self.0.values()
    }
}
