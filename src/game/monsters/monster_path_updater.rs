use crate::prelude::*;

struct CurrentPathTimerDetails {
    parent_sequence: TimerParentSequence,
    timer: EmittingTimer,
    movement_calculator: Entity,
}

pub struct MonsterPathUpdaterPlugin;

impl Plugin for MonsterPathUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (listen_for_monster_state_set_requests));
    }
}

fn listen_for_monster_state_set_requests(
    mut monster_state_set_listener: EventReader<MonserStateSetRequest>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    emitting_timers_with_sequence_parent: Query<(&EmittingTimer, &TimerParentSequence)>,
    timer_sequence_query: Query<&TimerSequence>,
    mut commands: Commands,
) {
    for request in monster_state_set_listener.read() {
        match request.0 {
            MonsterState::Chasing(entity_to_chase) => {
                monster.state = MonsterState::Chasing(entity_to_chase);
                set_path_to_chase_player(
                    &mut timer_fire_request_writer,
                    affecting_timer_calculators,
                    &emitting_timers_with_sequence_parent,
                    &timer_sequence_query,
                    &mut commands,
                );
            }
        }
    }
}

fn set_path_to_chase_player(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timers_with_sequence_parent: &Query<(&EmittingTimer, &TimerParentSequence)>,
    timer_sequence_query: &Query<&TimerSequence>,
    commands: &mut Commands,
) {
    if let Some(path_timer_details) = destroy_current_path_timer(
        affecting_timer_calculators,
        emitting_timers_with_sequence_parent,
        commands,
    ) {
        if let Ok(timer_sequence) =
            timer_sequence_query.get(path_timer_details.parent_sequence.parent_sequence)
        {
            if let Ok(timer_from_sequence) = timer_sequence
                .get_timer_by_index(path_timer_details.parent_sequence.index_in_sequence)
            {
                if timer_from_sequence != path_timer_details.timer {
                    despawn_recursive_notify_on_fail(
                        path_timer_details.movement_calculator,
                        "calculator when changing enemy path, since it's not the one from the source path",
                        commands,
                    );
                }
            }
        }
    }
}

fn destroy_current_path_timer(
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timers_with_sequence_parent: &Query<(&EmittingTimer, &TimerParentSequence)>,
    commands: &mut Commands,
) -> Option<CurrentPathTimerDetails> {
    if let Some(direct_line_movers) =
        affecting_timer_calculators.get(&TimerGoingEventType::Move(MovementType::InDirectLine))
    {
        for timer_and_calculator in direct_line_movers {
            if let Ok((timer, parent_sequence)) =
                emitting_timers_with_sequence_parent.get(timer_and_calculator.timer)
            {
                despawn_recursive_notify_on_fail(
                    timer_and_calculator.timer,
                    "timer when changing enemy path",
                    commands,
                );
                return Some(CurrentPathTimerDetails {
                    parent_sequence: *parent_sequence,
                    timer: *timer,
                    movement_calculator: timer_and_calculator.value_calculator,
                });
            }
        }
    }
    None
}
