use crate::{prelude::*, read_no_field_variant};
use bevy_light_2d::light::PointLight2d;

pub struct DynamicLightManagerPlugin;
impl Plugin for DynamicLightManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                set_light_intensity.in_set(TickingSystemSet::PostTicking),
                fade_light_on_game_over,
            ),
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

fn fade_light_on_game_over(
    mut game_over_listener: EventReader<GameEvent>,
    mut timer_fire_writer: EventWriter<TimerFireRequest>,
    lights: Query<(&PointLight2d, Entity)>,
    mut commands: Commands,
) {
    for _game_over in read_no_field_variant!(game_over_listener, GameEvent::GameOver) {
        for (light, entity) in &lights {
            let calculator = light_fade_upon_game_over_calculator(light.intensity, &mut commands);
            timer_fire_writer.send(TimerFireRequest {
                timer: EmittingTimer::new(
                    vec![TimerAffectedEntity {
                        affected_entity: entity,
                        value_calculator_entity: Some(calculator),
                    }],
                    vec![TimeMultiplierId::RealTime],
                    AGAIN_SCREEN_FADE_TIME / 2.0,
                    TimerDoneEventType::Nothing,
                ),
                parent_sequence: None,
            });
        }
    }
}

fn light_fade_upon_game_over_calculator(current_intensity: f32, commands: &mut Commands) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::IgnoreNewIfAssigned,
            ValueByInterpolation::from_goal_and_current(
                current_intensity,
                0.0,
                Interpolator::new(0.1),
            ),
            TimerGoingEventType::SetLightIntensity,
        ))
        .id()
}
