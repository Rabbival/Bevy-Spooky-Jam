use crate::prelude::*;

pub struct TranslationChangePlugin;

impl Plugin for TranslationChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_translation_update_requests.in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn listen_for_translation_update_requests(
    mut event_reader: EventReader<TimerGoingEvent<Vec3>>,
    mut transforms: Query<(&mut Transform, Option<&WorldBoundsWrapped>)>,
) {
    for event_from_timer in event_reader.read() {
        if let TimerGoingEventType::Move(MovementType::InDirectLine) = event_from_timer.event_type {
            match transforms.get_mut(event_from_timer.entity) {
                Ok((mut transform, maybe_world_bound)) => {
                    transform.translation += event_from_timer.value_delta;
                    if maybe_world_bound.is_some() {
                        transform.translation =
                            calculate_reach_beyond_screen_border(transform.translation);
                    }
                }
                Err(_) => {
                    print_error(
                        EntityError::EntityNotInQuery(&format!(
                            "couldn't fetch transform from query on translation update function",
                        )),
                        vec![LogCategory::Crucial, LogCategory::RequestNotFulfilled],
                    );
                }
            }
        }
    }
}
