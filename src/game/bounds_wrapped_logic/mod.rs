use crate::prelude::*;

pub mod bounds_wrapped_phantom;
pub mod consts;
pub mod phantom_manager;
pub mod phantom_spawner;

pub struct BoundsWrappedLogicPlugin;

impl Plugin for BoundsWrappedLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PhantomSpawnerPlugin, PhantomManagerPlugin));
    }
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
