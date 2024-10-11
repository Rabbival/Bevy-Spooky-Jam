use std::fmt::Debug;

use crate::prelude::*;

pub fn clamp_and_notify<T: PartialOrd + Debug + 'static>(value: T, min: T, max: T) -> T {
    if value < min {
        print_warning(
            format!(
                "value {:?} was below min: {:?},\n
                fixed to min.",
                value, min
            ),
            vec![
                LogCategory::ValueValidation,
                LogCategory::RequestNotFulfilled,
            ],
        );
        min
    } else if value > max {
        print_warning(
            format!(
                "value {:?} was above max: {:?},\n
                fixed to max.",
                value, max
            ),
            vec![
                LogCategory::ValueValidation,
                LogCategory::RequestNotFulfilled,
            ],
        );
        max
    } else {
        value
    }
}

pub fn truncated_if_at_limit<T: Debug>(vec: Vec<T>, max_count: usize) -> Vec<T> {
    if vec.len() > max_count {
        print_warning(
            format!(
                "{:?} reached max count {}, shortning to max",
                vec, max_count
            ),
            vec![
                LogCategory::ValueValidation,
                LogCategory::RequestNotFulfilled,
            ],
        );
        vec.into_iter().take(max_count).collect()
    } else {
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clamp() {
        let original_value = 3.0;
        let bigger_value = 4.0;
        let smaller_value = -1.0;

        let original_outcome = clamp_and_notify(original_value, smaller_value, bigger_value);
        let bigger_outcome = clamp_and_notify(original_value, bigger_value, bigger_value);
        let smaller_outcome = clamp_and_notify(original_value, smaller_value, smaller_value);

        assert_eq!(original_value, original_outcome);
        assert_eq!(bigger_value, bigger_outcome);
        assert_eq!(smaller_value, smaller_outcome);
    }

    #[test]
    fn test_truncated_if_at_limit() {
        let vec = vec![1, 2, 3];

        let truncated_vec = truncated_if_at_limit(vec.clone(), 2);
        let cloned_vec = truncated_if_at_limit(vec.clone(), 4);

        assert_eq!(truncated_vec, vec!(1, 2));
        assert_eq!(vec, cloned_vec);
    }
}
