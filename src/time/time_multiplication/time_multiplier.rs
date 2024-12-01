use crate::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct TimeMultiplier {
    id: TimeMultiplierId,
    value: f32,
    maybe_overriding_value: Option<f32>,
    changeable: bool,
}

impl TimeMultiplier {
    pub fn new(id: TimeMultiplierId, value: f32, changeable: bool) -> Self {
        let clamped_value = clamp_and_notify(value, MIN_TIME_MULTIPLIER, MAX_TIME_MULTIPLIER);
        Self {
            id,
            value: clamped_value,
            maybe_overriding_value: None,
            changeable,
        }
    }

    pub fn id(&self) -> TimeMultiplierId {
        self.id
    }

    pub fn value(&self) -> f32 {
        if let Some(overriding_value) = self.maybe_overriding_value {
            overriding_value
        } else {
            self.value
        }
    }

    pub fn changeable(&self) -> bool {
        self.changeable
    }

    pub fn update_value(&mut self, value_delta: f32) {
        if self.changeable {
            self.value += value_delta;
        } else {
            print_warning(
                TimeRelatedError::AttemptedToChangeFixedTimeMultiplier(self.id),
                vec![LogCategory::RequestNotFulfilled, LogCategory::Time],
            )
        }
    }

    pub fn set_overriding_value(&mut self, new_value: Option<f32>) {
        self.maybe_overriding_value = new_value;
    }

    pub fn set_value(&mut self, new_value: f32) {
        self.value = new_value;
    }
}
