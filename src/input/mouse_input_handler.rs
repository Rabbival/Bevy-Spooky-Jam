use crate::prelude::*;
use crate::single_else_return;

#[derive(Resource, Default)]
pub struct CursorWorldPosition(pub Vec2);

pub struct MouseInputHandlerPlugin;

impl Plugin for MouseInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldPosition>().add_systems(
            Update,
            update_cursor_in_game_world.in_set(InputSystemSet::Listening),
        );
    }
}

fn update_cursor_in_game_world(
    mut cursor: ResMut<CursorWorldPosition>,
    windows: Query<&Window>,
    camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = single_else_return!(windows);
    let (camera, transform) = single_else_return!(camera);

    if let Some(screen_position) = window.cursor_position() {
        let maybe_world_position = camera
            .viewport_to_world(transform, screen_position)
            .map(|ray| ray.origin.truncate());
        if let Some(world_position) = maybe_world_position {
            cursor.0 = world_position;
        }
    }
}
