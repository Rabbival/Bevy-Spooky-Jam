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
                Ok((mut transform, maybe_world_bounds)) => {
                    transform.translation = calcualte_updated_translation(
                        &mut transform,
                        event_from_timer.value_delta,
                        maybe_world_bounds.is_some(),
                    );
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

fn calcualte_updated_translation(
    transform: &Transform,
    delta: Vec3,
    clamp_to_viewport: bool,
) -> Vec3 {
    let half_screen_size = WINDOW_SIZE_IN_PIXELS / 2.0;
    let mut updated_translation = transform.translation + delta;
    if clamp_to_viewport {
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
    }
    updated_translation
}
