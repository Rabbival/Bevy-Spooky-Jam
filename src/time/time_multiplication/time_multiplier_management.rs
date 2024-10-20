use strum::IntoEnumIterator;

use crate::prelude::*;

pub struct TimeMultiplierPlugin;

impl Plugin for TimeMultiplierPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, initialize_time_multipliers)
            .add_systems(
                Update,
                (
                    listen_for_time_multiplier_update_requests,
                    listen_for_time_multiplier_set_requests,
                )
                    .in_set(TickingSystemSet::PostTicking),
            );
    }
}

fn initialize_time_multipliers(mut commands: Commands) {
    for time_multiplier_id in TimeMultiplierId::iter() {
        commands.spawn((
            time_multiplier_id.to_initial_properties(),
            AffectingTimerCalculators::default(),
            DoNotDestroyOnRestart,
        ));
    }
}

fn listen_for_time_multiplier_update_requests(
    mut event_from_timer_reader: EventReader<TimerGoingEvent<f32>>,
    mut time_multipliers: Query<(&mut TimeMultiplier, Entity)>,
) {
    for event_from_timer in event_from_timer_reader.read() {
        if let TimerGoingEventType::ChangeTimeMultiplierSpeed = event_from_timer.event_type {
            if let Ok((mut multiplier, _)) = time_multipliers.get_mut(event_from_timer.entity) {
                multiplier.update_value(event_from_timer.value_delta);
            }
        }
    }
}

fn listen_for_time_multiplier_set_requests(
    mut time_multiplier_set_request_reader: EventReader<SetTimeMultiplier>,
    mut timer_fire_event_writer: EventWriter<TimerFireRequest>,
    time_multipliers: Query<(&TimeMultiplier, Entity)>,
    mut commands: Commands,
) {
    for time_multiplier_set_request in time_multiplier_set_request_reader.read() {
        if let Err(timer_error) = fire_time_multiplier_changers(
            &mut timer_fire_event_writer,
            &time_multipliers,
            time_multiplier_set_request,
            &mut commands,
        ) {
            print_warning(
                timer_error,
                vec![LogCategory::RequestNotFulfilled, LogCategory::Time],
            );
        }
    }
}

fn fire_time_multiplier_changers(
    timer_fire_event_writer: &mut EventWriter<TimerFireRequest>,
    time_multipliers: &Query<(&TimeMultiplier, Entity)>,
    multiplier_set_request: &SetTimeMultiplier,
    commands: &mut Commands,
) -> Result<(), TimeRelatedError> {
    for (multiplier, multiplier_entity) in time_multipliers {
        if multiplier.id() == multiplier_set_request.multiplier_id {
            if multiplier.changeable() {
                spawn_calculator_and_fire_multiplier_changer(
                    timer_fire_event_writer,
                    multiplier_set_request,
                    multiplier_entity,
                    multiplier,
                    commands,
                );
                return Ok(());
            } else {
                return Err(TimeRelatedError::AttemptedToChangeFixedTimeMultiplier(
                    multiplier_set_request.multiplier_id,
                ));
            }
        }
    }
    Err(TimeRelatedError::TimeMultiplierNotFound(
        multiplier_set_request.multiplier_id,
    ))
}

fn spawn_calculator_and_fire_multiplier_changer(
    timer_fire_event_writer: &mut EventWriter<TimerFireRequest>,
    multiplier_set_request: &SetTimeMultiplier,
    multiplier_entity: Entity,
    multiplier: &TimeMultiplier,
    commands: &mut Commands,
) {
    let value_calculator_id = commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::KeepNewTimer,
            ValueByInterpolation::<f32>::from_goal_and_current(
                multiplier.value(),
                multiplier_set_request.new_multiplier,
                Interpolator::default(),
            ),
            TimerGoingEventType::ChangeTimeMultiplierSpeed,
        ))
        .id();
    timer_fire_event_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: multiplier_entity,
                value_calculator_entity: Some(value_calculator_id),
            }],
            vec![],
            multiplier_set_request.duration,
            TimerDoneEventType::default(),
        ),
        parent_sequence: None,
    });
}
