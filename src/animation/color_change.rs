use crate::prelude::*;

pub struct ColorChangePlugin;

impl Plugin for ColorChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_color_update_requests.in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn listen_for_color_update_requests(
    mut event_reader: EventReader<TimerGoingEvent<f32>>,
    mut colorables_query: Query<(Option<&mut Sprite>, Option<&mut Text>)>,
) {
    for event_from_timer in event_reader.read() {
        if let TimerGoingEventType::SetAlpha = event_from_timer.event_type {
            match colorables_query.get_mut(event_from_timer.entity) {
                Ok((maybe_sprite, maybe_text)) => {
                    if let Some(mut sprite) = maybe_sprite {
                        let current_alpha = sprite.color.alpha();
                        sprite
                            .color
                            .set_alpha(current_alpha + event_from_timer.value_delta);
                    }
                    if let Some(mut text) = maybe_text {
                        let color_ref = &mut text.sections[0].style.color;
                        let current_alpha = color_ref.alpha();
                        color_ref.set_alpha(current_alpha + event_from_timer.value_delta);
                    }
                }
                Err(_) => {
                    print_error(
                        EntityError::EntityNotInQuery(
                            "couldn't fetch colorable from query on alpha change function",
                        ),
                        vec![LogCategory::Crucial, LogCategory::RequestNotFulfilled],
                    );
                }
            }
        }
    }
}
