use crate::prelude::*;
use bevy_light_2d::light::PointLight2d;

pub struct DynamicLightManagerPlugin;
impl Plugin for DynamicLightManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            set_light_intensity.in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn set_light_intensity(
    mut event_reader: EventReader<TimerGoingEvent<f32>>,
    mut lights: Query<&mut PointLight2d>,
) {
    for event_from_timer in event_reader.read() {
        if let TimerGoingEventType::SetLightIntensity = event_from_timer.event_type {
            match lights.get_mut(event_from_timer.entity) {
                Ok(mut light) => {
                    light.intensity += event_from_timer.value_delta;
                }
                Err(_) => {
                    print_error(
                        EntityError::EntityNotInQuery(
                            "couldn't fetch light from query on light intensity update function",
                        ),
                        vec![LogCategory::RequestNotFulfilled],
                    );
                }
            }
        }
    }
}
