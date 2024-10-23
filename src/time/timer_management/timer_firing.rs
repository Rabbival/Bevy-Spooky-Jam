use crate::{plugin_for_implementors_of_trait, prelude::*};

pub struct TimerFiringPlugin;

impl Plugin for TimerFiringPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                listen_for_emitting_timer_firing_requests,
                reset_calculators_if_part_of_looping_sequence,
            )
                .in_set(TickingSystemSet::PreTickingEarlyPreperations),
        );
    }
}

plugin_for_implementors_of_trait!(TimerFiringGenericPlugin, Numeric);

impl<T: Numeric> Plugin for TimerFiringGenericPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_update_affected_entities_after_timer_birth_requests::<T>
                .in_set(TickingSystemSet::PreTickingPreperations),
        );
    }
}

pub fn listen_for_emitting_timer_firing_requests(
    mut event_reader: EventReader<TimerFireRequest>,
    mut event_writer: EventWriter<UpdateAffectedEntitiesAfterTimerBirth>,
    mut commands: Commands,
) {
    for timer_fire_request in event_reader.read() {
        let timer_entity = if let Some(parent_sequence) = timer_fire_request.parent_sequence {
            commands
                .spawn((timer_fire_request.timer, parent_sequence))
                .id()
        } else {
            commands.spawn(timer_fire_request.timer).id()
        };
        event_writer.send(UpdateAffectedEntitiesAfterTimerBirth {
            timer_entity,
            newborn_timer: timer_fire_request.timer,
        });
    }
}

pub fn reset_calculators_if_part_of_looping_sequence(
    mut event_reader: EventReader<TimerFireRequest>,
    mut calculator_event_channel_writer: EventWriter<ValueCalculatorRequest>,
    timer_sequence_query: Query<&TimerSequence>,
) {
    for timer_fire_request in event_reader.read() {
        if let Some(parent_sequence) = timer_fire_request.parent_sequence {
            if let Ok(fetched_sequence) = timer_sequence_query.get(parent_sequence.parent_sequence)
            {
                if fetched_sequence.loop_back_to_start {
                    for calculator_entity in timer_fire_request.timer.calculator_entities_iter() {
                        calculator_event_channel_writer
                            .send(ValueCalculatorRequest::Initialize(calculator_entity));
                    }
                }
            }
        }
    }
}

pub fn listen_for_update_affected_entities_after_timer_birth_requests<T: Numeric>(
    mut event_reader: EventReader<UpdateAffectedEntitiesAfterTimerBirth>,
    mut remove_from_timer_entities_writer: EventWriter<RemoveFromTimerAffectedEntities>,
    mut affected_by_timer_query: Query<&mut AffectingTimerCalculators>,
    timer_calculators: Query<&GoingEventValueCalculator<T>>,
    emitting_timers: Query<&EmittingTimer>,
    mut commands: Commands,
) {
    for affected_entities_update_request in event_reader.read() {
        let newborn_timer_entity = affected_entities_update_request.timer_entity;
        let newborn_timer = affected_entities_update_request.newborn_timer;
        for timer_affected_entity in newborn_timer.affected_entities.iter() {
            if let Some(value_calculator_entity) = timer_affected_entity.value_calculator_entity {
                if let Ok(value_calculator) = timer_calculators.get(value_calculator_entity) {
                    match affected_by_timer_query.get_mut(timer_affected_entity.affected_entity) {
                        Ok(mut affecting_timer_calculators) => {
                            set_active_calculator_and_destroy_inactive(
                                &mut remove_from_timer_entities_writer,
                                &mut affecting_timer_calculators,
                                TimerAndCalculator {
                                    timer: newborn_timer_entity,
                                    value_calculator: value_calculator_entity,
                                },
                                value_calculator,
                                &emitting_timers,
                                &mut commands,
                            );
                        }
                        Err(_) => print_warning(
                            EntityError::EntityNotInQuery(
                                "couldn't find entity in affecting timers component query upon timer firing",
                            ),
                            vec![LogCategory::RequestNotFulfilled, LogCategory::Time],
                        ),
                    }
                }
            }
        }
    }
}

fn set_active_calculator_and_destroy_inactive<T: Numeric>(
    remove_from_timer_entities_writer: &mut EventWriter<RemoveFromTimerAffectedEntities>,
    affecting_timer_calculators: &mut AffectingTimerCalculators,
    newborn_timer_and_calculator: TimerAndCalculator,
    value_calculator: &GoingEventValueCalculator<T>,
    emitting_timers: &Query<&EmittingTimer>,
    commands: &mut Commands,
) {
    let maybe_timers_to_remove_from = affecting_timer_calculators.insert_get_rejected_value(
        value_calculator.going_event_type(),
        newborn_timer_and_calculator,
        value_calculator.set_policy,
    );
    if let Some(timers_to_remove_from) = maybe_timers_to_remove_from {
        for timer_to_remove_from in timers_to_remove_from {
            destory_inactive_and_send_removal_request(
                remove_from_timer_entities_writer,
                timer_to_remove_from,
                emitting_timers,
                commands,
            );
        }
    }
}

fn destory_inactive_and_send_removal_request(
    remove_from_timer_entities_writer: &mut EventWriter<RemoveFromTimerAffectedEntities>,
    remove_from_and_destroy: TimerAndCalculator,
    emitting_timers: &Query<&EmittingTimer>,
    commands: &mut Commands,
) {
    despawn_recursive_notify_on_fail(
        remove_from_and_destroy.value_calculator,
        "calculator when choosing an affecting timer",
        commands,
    );
    if let Ok(timer) = emitting_timers.get(remove_from_and_destroy.timer) {
        if let Some(affected_entity) = timer
            .affected_entities
            .get_by_calculator_entity(remove_from_and_destroy.value_calculator)
        {
            remove_from_timer_entities_writer.send(RemoveFromTimerAffectedEntities {
                timer_entity: remove_from_and_destroy.timer,
                entity_to_remove: affected_entity,
            });
        }
    }
}
