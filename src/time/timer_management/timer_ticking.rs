use crate::prelude::*;

pub struct TimerTickingPlugin;

impl Plugin for TimerTickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            tick_emitting_timers.in_set(TickingSystemSet::TimerTicking),
        );
    }
}

pub fn tick_emitting_timers(
    mut calculation_requests_writer: EventWriter<CalculateAndSendGoingEvent>,
    mut extract_affected_and_send_done_event_writer: EventWriter<TimerDoneEvent>,
    mut timers: Query<(&mut EmittingTimer, Option<&TimerParentSequence>, Entity)>,
    time_multipliers: Query<&TimeMultiplier>,
    time: Res<Time>,
) {
    let time_delta = time.delta_seconds();
    for (mut timer, maybe_parent_sequence, timer_entity) in &mut timers {
        let modified_time_delta =
            time_delta * calculate_time_multiplier(&time_multipliers, timer.time_multipliers);
        tick_emitting_timer_and_send_events(
            &mut calculation_requests_writer,
            &mut extract_affected_and_send_done_event_writer,
            modified_time_delta,
            &mut timer,
            timer_entity,
            maybe_parent_sequence.copied(),
        );
    }
}

fn calculate_time_multiplier<const N: usize>(
    time_multipliers: &Query<&TimeMultiplier>,
    multipliers_timer_subscribes_to: VecBasedArray<TimeMultiplierId, N>,
) -> f32 {
    let mut calculated_multiplier = DEFAULT_TIME_MULTIPLIER;
    for multiplier_id_from_timer in multipliers_timer_subscribes_to.iter() {
        for time_multiplier in time_multipliers {
            if time_multiplier.id() == multiplier_id_from_timer {
                calculated_multiplier *= time_multiplier.value();
            }
        }
    }
    calculated_multiplier
}

fn tick_emitting_timer_and_send_events(
    calculation_requests_writer: &mut EventWriter<CalculateAndSendGoingEvent>,
    extract_affected_and_send_done_event_writer: &mut EventWriter<TimerDoneEvent>,
    time_to_tick: f32,
    timer: &mut EmittingTimer,
    timer_entity: Entity,
    maybe_parent_sequence: Option<TimerParentSequence>,
) {
    if let Some(normalized_progress) = timer.tick_and_get_normalized_progress(time_to_tick) {
        for affected_entity in timer.affected_entities.iter() {
            if let Some(calculator_entity) = affected_entity.value_calculator_entity {
                calculation_requests_writer.send(CalculateAndSendGoingEvent {
                    going_event_value_calculator: calculator_entity,
                    affected_entity: affected_entity.affected_entity,
                    normalized_progress,
                });
            }
        }
        if timer.finished() {
            extract_affected_and_send_done_event_writer.send(TimerDoneEvent {
                event_type: timer.send_once_done,
                affected_entities: timer.affected_entities,
                timer_entity,
                timer_parent_sequence: maybe_parent_sequence,
            });
        }
    }
}
