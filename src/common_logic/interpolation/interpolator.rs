use crate::prelude::{clamp_and_notify, Numeric};

#[derive(Debug, Clone, Copy)]
pub struct Interpolator {
    pub power: f32,
}

impl Interpolator {
    pub fn new(power: f32) -> Self {
        Self { power }
    }

    pub fn calculate<T: Numeric>(&self, origin: T, delta: T, normalized_progress: f32) -> T {
        let clamped_normalized_progress = clamp_and_notify(normalized_progress, 0.0, 1.0);
        origin + delta * clamped_normalized_progress.powf(self.power)
    }
}

impl Default for Interpolator {
    fn default() -> Self {
        Self { power: 1.0 }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use super::*;

    #[test]
    fn test_interpolator_for_out_of_bounds_progress() {
        let interpolator = Interpolator::default();
        let origin_value = 1.0;
        let value_delta = 41.0;
        let above_max_limit_progress = PI;
        let below_min_limit_progress = -PI;

        let endpoint = interpolator.calculate(origin_value, value_delta, above_max_limit_progress);
        let startpoint =
            interpolator.calculate(origin_value, value_delta, below_min_limit_progress);

        assert_eq!(endpoint, origin_value + value_delta);
        assert_eq!(startpoint, origin_value);
    }

    #[test]
    fn test_linear_interpolator() {
        let linear_interpolator = Interpolator::new(1.0);
        let origin_value = 0.0;
        let value_delta = 1.0;
        let progress = 0.5;

        let midpoint = linear_interpolator.calculate(origin_value, value_delta, progress);

        assert_eq!(midpoint, 0.5);
    }

    #[test]
    fn test_parabolic_interpolator() {
        let parabolic_interpolator = Interpolator::new(2.0);
        let origin_value = 1.0;
        let value_delta = 1.0;
        let progress = 0.5;

        let midpoint = parabolic_interpolator.calculate(origin_value, value_delta, progress);

        assert_eq!(midpoint, 1.25);
    }

    #[test]
    fn test_radical_interpolator() {
        let radical_interpolator = Interpolator::new(0.5);
        let origin_value = 1.0;
        let value_delta = 1.0;
        let progress = 0.25;

        let quarter_from_start_point =
            radical_interpolator.calculate(origin_value, value_delta, progress);

        assert_eq!(quarter_from_start_point, 1.5);
    }
}
