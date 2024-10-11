use crate::prelude::*;

#[derive(Debug)]
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
