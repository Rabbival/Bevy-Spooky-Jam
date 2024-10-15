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
    mut sprite_query: Query<&mut Sprite>,
) {
    for event_from_timer in event_reader.read() {
        if let TimerGoingEventType::SetAlpha = event_from_timer.event_type {
            match sprite_query.get_mut(event_from_timer.entity) {
                Ok(mut sprite) => {
                    let current_alpha = sprite.color.alpha();
                    sprite
                        .color
                        .set_alpha(current_alpha + event_from_timer.value_delta);
                }
                Err(_) => {
                    print_error(
                        EntityError::EntityNotInQuery(
                            "couldn't fetch sprite from query on alpha change function",
                        ),
                        vec![LogCategory::Crucial, LogCategory::RequestNotFulfilled],
                    );
                }
            }
        }
    }
}
