use crate::prelude::*;

pub struct WorldBoundPlugin;

impl Plugin for WorldBoundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, handle_world_bounds_wrap_system);
    }
}

fn handle_world_bounds_wrap_system(
    mut world_bounds_wrapped_transform_query: Query<&mut Transform, With<WorldBoundsWrapped>>,
) {
    let half_screen_size = WINDOW_SIZE_IN_PIXELS / 2.0;
    for mut transform in world_bounds_wrapped_transform_query.iter_mut() {
        if transform.translation.x < -half_screen_size {
            transform.translation.x = half_screen_size;
        }
        if transform.translation.x > half_screen_size {
            transform.translation.x = -half_screen_size;
        }
        if transform.translation.y < -half_screen_size {
            transform.translation.y = half_screen_size;
        }
        if transform.translation.y > half_screen_size {
            transform.translation.y = -half_screen_size;
        }
    }
}
