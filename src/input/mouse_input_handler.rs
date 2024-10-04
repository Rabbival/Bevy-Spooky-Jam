use crate::prelude::*;
use crate::single_else_return;

#[derive(Resource, Default)]
pub struct CursorWorldPosition(pub Vec2);

pub struct MouseInputHandlerPlugin;

impl Plugin for MouseInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorWorldPosition>().add_systems(
            Update,
            (update_cursor_in_game_world, listen_for_mouse_clicks)
                .chain()
                .in_set(InputSystemSet::Listening),
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
        let world_position = camera
            .viewport_to_world(transform, screen_position)
            .unwrap()
            .origin
            .truncate();
        cursor.0 = world_position;
    }
}

fn listen_for_mouse_clicks(
    mut orb_event_writer: EventWriter<OrbEvent>,
    mouse: Res<ButtonInput<MouseButton>>,
    cursor_position: Res<CursorWorldPosition>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        orb_event_writer.send(OrbEvent::SpawnOrb(cursor_position.0));
    }
    if mouse.just_pressed(MouseButton::Right) {
        orb_event_writer.send(OrbEvent::CollectAllOrbs(cursor_position.0));
    }
}
