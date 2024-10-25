use crate::prelude::*;

pub struct ScaleChangePlugin;

impl Plugin for ScaleChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_scale_update_requests.in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn listen_for_scale_update_requests(
    mut event_reader: EventReader<TimerGoingEvent<Vec3>>,
    mut transforms: Query<&mut Transform>,
) {
    for event_from_timer in event_reader.read() {
        if let TimerGoingEventType::Scale = event_from_timer.event_type {
            match transforms.get_mut(event_from_timer.entity) {
                Ok(mut transform) => {
                    transform.scale += event_from_timer.value_delta;
                }
                Err(_) => {
                    print_error(
                        EntityError::EntityNotInQuery(
                            "couldn't fetch transform from query on scale update function",
                        ),
                        vec![LogCategory::RequestNotFulfilled],
                    );
                }
            }
        }
    }
}
