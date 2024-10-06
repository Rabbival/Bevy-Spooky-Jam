use crate::prelude::*;

pub struct ClosestSecondVecIncludingBeyondScreen {
    pub vec: Vec3,
    pub distance: f32,
}

pub fn calculate_reach_beyond_screen_border(original_translation: Vec3) -> Vec3 {
    let half_screen_size = WINDOW_SIZE_IN_PIXELS / 2.0;
    let mut updated_translation = original_translation;
    if updated_translation.x < -half_screen_size {
        updated_translation.x = half_screen_size;
    }
    if updated_translation.x > half_screen_size {
        updated_translation.x = -half_screen_size;
    }
    if updated_translation.y < -half_screen_size {
        updated_translation.y = half_screen_size;
    }
    if updated_translation.y > half_screen_size {
        updated_translation.y = -half_screen_size;
    }
    updated_translation
}

pub fn closer_beyond_screen_value(original_value: f32) -> f32 {
    if original_value > 0.0 {
        original_value - WINDOW_SIZE_IN_PIXELS
    } else {
        WINDOW_SIZE_IN_PIXELS + original_value
    }
}

pub fn x_closer_beyond_screen(original: Vec3) -> Vec3 {
    Vec3::new(
        closer_beyond_screen_value(original.x),
        original.y,
        original.z,
    )
}

pub fn y_closer_beyond_screen(original: Vec3) -> Vec3 {
    Vec3::new(
        original.x,
        closer_beyond_screen_value(original.y),
        original.z,
    )
}

pub fn fully_negated_closer_beyond_screen(original: Vec3) -> Vec3 {
    Vec3::new(
        closer_beyond_screen_value(original.x),
        closer_beyond_screen_value(original.y),
        original.z,
    )
}

pub fn calculate_distance_including_through_screen_border(
    first_location: Vec3,
    second_location: Vec3,
) -> ClosestSecondVecIncludingBeyondScreen {
    let mut minimal_vec_and_distance = ClosestSecondVecIncludingBeyondScreen {
        vec: second_location,
        distance: Vec3::distance(first_location, second_location),
    };
    let x_negated = first_location.x * second_location.x < 0.0;
    let y_negated = first_location.y * second_location.y < 0.0;
    if x_negated && y_negated {
        if let Some(new_minimal) = get_vector_and_distance_if_closer(
            minimal_vec_and_distance.distance,
            first_location,
            fully_negated_closer_beyond_screen(second_location),
        ) {
            minimal_vec_and_distance = new_minimal;
        }
    }
    if x_negated {
        if let Some(new_minimal) = get_vector_and_distance_if_closer(
            minimal_vec_and_distance.distance,
            first_location,
            x_closer_beyond_screen(second_location),
        ) {
            minimal_vec_and_distance = new_minimal;
        }
    }
    if y_negated {
        if let Some(new_minimal) = get_vector_and_distance_if_closer(
            minimal_vec_and_distance.distance,
            first_location,
            y_closer_beyond_screen(second_location),
        ) {
            minimal_vec_and_distance = new_minimal;
        }
    }
    minimal_vec_and_distance
}

fn get_vector_and_distance_if_closer(
    original_min: f32,
    first_location: Vec3,
    second_location: Vec3,
) -> Option<ClosestSecondVecIncludingBeyondScreen> {
    let new_minimal_distance = min(
        original_min,
        Vec3::distance(first_location, second_location),
    );
    if new_minimal_distance < original_min {
        Some(ClosestSecondVecIncludingBeyondScreen {
            vec: second_location,
            distance: new_minimal_distance,
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closer_beyond_screen_value() {
        let x_absolute = 42.0;

        let behind_the_left_end = closer_beyond_screen_value(x_absolute);
        let behind_the_right_end = closer_beyond_screen_value(x_absolute * -1.0);

        assert_eq!(behind_the_left_end, -(WINDOW_SIZE_IN_PIXELS - x_absolute));
        assert_eq!(behind_the_right_end, WINDOW_SIZE_IN_PIXELS - x_absolute);
    }

    #[test]
    fn test_calculate_distance_including_through_screen_border() {
        let dist = WINDOW_SIZE_IN_PIXELS / 4.0;
        let bigger_dist = (WINDOW_SIZE_IN_PIXELS / 8.0) * 3.0;
        let vector = Vec3::new(dist, dist, 0.0);
        let x_negated_closer_vector = Vec3::new(-dist, dist, 0.0);
        let x_negated_further_vector = Vec3::new(-bigger_dist, dist, 0.0);
        let fully_negated_closer_vector = Vec3::new(-dist, -dist, 0.0);
        let fully_negated_further_vector = Vec3::new(-bigger_dist, -bigger_dist, 0.0);
        let expected_fully_negated_closer_length = ((dist * 2.0).powf(2.0) * 2.0).sqrt();
        let expected_fully_negated_further_length = ((dist * 1.5).powf(2.0) * 2.0).sqrt();

        let closer_one_axis =
            calculate_distance_including_through_screen_border(x_negated_closer_vector, vector);
        let closer_one_axis_flipped =
            calculate_distance_including_through_screen_border(vector, x_negated_closer_vector);
        let further_one_axis =
            calculate_distance_including_through_screen_border(x_negated_further_vector, vector);
        let further_one_axis_flipped =
            calculate_distance_including_through_screen_border(vector, x_negated_further_vector);
        let closer_two_axis =
            calculate_distance_including_through_screen_border(fully_negated_closer_vector, vector);
        let closer_two_axis_flipped =
            calculate_distance_including_through_screen_border(vector, fully_negated_closer_vector);
        let further_two_axis = calculate_distance_including_through_screen_border(
            fully_negated_further_vector,
            vector,
        );
        let further_two_axis_flipped = calculate_distance_including_through_screen_border(
            vector,
            fully_negated_further_vector,
        );

        assert_eq!(closer_one_axis.distance, WINDOW_SIZE_IN_PIXELS / 2.0);
        assert_eq!(
            closer_one_axis_flipped.distance,
            WINDOW_SIZE_IN_PIXELS / 2.0
        );
        assert_eq!(
            further_one_axis.distance,
            (WINDOW_SIZE_IN_PIXELS / 8.0) * 3.0
        );
        assert_eq!(
            further_one_axis_flipped.distance,
            (WINDOW_SIZE_IN_PIXELS / 8.0) * 3.0
        );
        assert_eq!(
            closer_two_axis.distance,
            expected_fully_negated_closer_length
        );
        assert_eq!(
            closer_two_axis_flipped.distance,
            expected_fully_negated_closer_length
        );
        assert_eq!(
            further_two_axis.distance,
            expected_fully_negated_further_length
        );
        assert_eq!(
            further_two_axis_flipped.distance,
            expected_fully_negated_further_length
        );
    }
}
