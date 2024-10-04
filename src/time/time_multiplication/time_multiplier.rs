use crate::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct TimeMultiplier {
    id: TimeMultiplierId,
    value: f32,
    changeable: bool,
}

impl TimeMultiplier {
    pub fn new(id: TimeMultiplierId, value: f32, changeable: bool) -> Self {
        let clamped_value = clamp_and_notify(value, MIN_TIME_MULTIPLIER, MAX_TIME_MULTIPLIER);
        Self {
            id,
            value: clamped_value,
            changeable,
        }
    }

    pub fn id(&self) -> TimeMultiplierId {
        self.id
    }

    pub fn value(&self) -> f32 {
        self.value
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
}
