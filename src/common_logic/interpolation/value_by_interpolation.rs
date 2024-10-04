use crate::prelude::*;

#[derive(Default, Debug, Clone, Copy)]
pub struct ValueByInterpolation<T: Numeric> {
    original_value: T,
    delta: T,
    interpolator: Interpolator,
    previous_value: T,
}

impl<T: Numeric> ValueByInterpolation<T> {
    pub fn new(original_value: T, delta: T, interpolator: Interpolator) -> Self {
        Self {
            original_value,
            delta,
            interpolator,
            previous_value: original_value,
        }
    }

    pub fn from_goal_and_current(
        original_value: T,
        goal_value: T,
        interpolator: Interpolator,
    ) -> Self {
        Self::new(original_value, goal_value - original_value, interpolator)
    }

    pub fn calculate_delta(&mut self, normalized_progress: f32) -> T {
        let current_value =
            self.interpolator
                .calculate(self.original_value, self.delta, normalized_progress);
        let delta = current_value - self.previous_value;
        self.previous_value = current_value;
        delta
    }

    pub fn initialize_previous_value(&mut self) {
        self.previous_value = self.original_value;
    }
}
