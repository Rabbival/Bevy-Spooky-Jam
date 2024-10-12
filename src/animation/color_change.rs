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
    color_handles: Query<&Handle<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for event_from_timer in event_reader.read() {
        if let TimerGoingEventType::SetAlpha = event_from_timer.event_type {
            match color_handles.get(event_from_timer.entity) {
                Ok(handle) => {
                    //DEBUG
                    info!("setting alpha");

                    if let Some(material) = materials.get_mut(handle.id()) {
                        let current_alpha = material.color.alpha();
                        material
                            .color
                            .set_alpha(current_alpha + event_from_timer.value_delta);
                    }
                }
                Err(_) => {
                    print_error(
                        EntityError::EntityNotInQuery(
                            "couldn't fetch color handle from query on alpha change function",
                        ),
                        vec![LogCategory::Crucial, LogCategory::RequestNotFulfilled],
                    );
                }
            }
        }
    }
}
