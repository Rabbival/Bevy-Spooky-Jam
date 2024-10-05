use crate::prelude::*;

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

pub fn calculate_distance_including_through_screen_border(
    first_location: Vec3,
    second_location: Vec3,
) -> f32 {
    let mut minimal_distance = Vec3::distance(first_location, second_location);
    let x_polarized = first_location.x * second_location.x < 0.0;
    let y_polarized = first_location.y * second_location.y < 0.0;
    if x_polarized && y_polarized {
        minimal_distance = min(
            minimal_distance,
            Vec3::distance(
                first_location,
                Vec3::new(
                    closer_beyond_screen_value(second_location.x),
                    closer_beyond_screen_value(second_location.y),
                    0.0,
                ),
            ),
        );
    }
    if x_polarized {
        minimal_distance = min(
            minimal_distance,
            Vec3::distance(
                first_location,
                Vec3::new(
                    closer_beyond_screen_value(second_location.x),
                    second_location.y,
                    0.0,
                ),
            ),
        );
    }
    if y_polarized {
        minimal_distance = min(
            minimal_distance,
            Vec3::distance(
                first_location,
                Vec3::new(
                    second_location.x,
                    closer_beyond_screen_value(second_location.y),
                    0.0,
                ),
            ),
        );
    }

    minimal_distance
}
