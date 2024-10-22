use soul_bomb_monster_hunt::prelude::*;

mod test_dependencies;

#[derive(Resource)]
struct ShouldSequenceLoop(Option<bool>);

struct SystemStatusAtThatPoint {
    value_calculators_count: usize,
    timers_count: usize,
    timer_sequences_count: usize,
}

const TIMER_DURATION_IN_SECONDS: f32 = 60.0;

#[test]
fn looping_sequence_timer_test() {
    timer_sequence_test(true);
}

#[test]
fn non_looping_sequence_test() {
    timer_sequence_test(false);
}

fn timer_sequence_test(looping_sequence: bool) {
    let mut app = test_dependencies::get_app_with_resources_and_events();
    app.insert_resource::<ShouldSequenceLoop>(ShouldSequenceLoop(Some(looping_sequence)))
        .add_systems(
            Update,
            (
                spawn_sequence,
                tick_emitting_timers,
                listen_for_emitting_timer_firing_requests,
                (listen_for_done_sequence_timers, clear_done_timers),
            )
                .chain(),
        );

    app.update();
    let count_after_creation = count_entities_in_world(&mut app);
    test_dependencies::time_dependencies::fast_forward(&mut app, TIMER_DURATION_IN_SECONDS);
    app.update();
    app.update();
    let count_after_first_timer_done = count_entities_in_world(&mut app);
    test_dependencies::time_dependencies::fast_forward(&mut app, TIMER_DURATION_IN_SECONDS);
    app.update();
    app.update();
    let count_after_last_timer_done = count_entities_in_world(&mut app);

    assert_sequence_was_still_going(count_after_creation);
    assert_sequence_was_still_going(count_after_first_timer_done);
    if looping_sequence {
        assert_sequence_was_still_going(count_after_last_timer_done);
    } else {
        assert_sequence_stopped_cleared_at_that_point(count_after_last_timer_done);
    }
}

fn count_entities_in_world(app: &mut App) -> SystemStatusAtThatPoint {
    SystemStatusAtThatPoint {
        value_calculators_count: count_f32_going_event_value_calculators(app),
        timers_count: count_emitting_timers(app),
        timer_sequences_count: count_timer_sequences(app),
    }
}

fn assert_sequence_was_still_going(system_status_then: SystemStatusAtThatPoint) {
    assert_eq!(system_status_then.value_calculators_count, 2);
    assert_eq!(system_status_then.timers_count, 1);
    assert_eq!(system_status_then.timer_sequences_count, 1);
}

fn assert_sequence_stopped_cleared_at_that_point(system_status_then: SystemStatusAtThatPoint) {
    assert_eq!(system_status_then.value_calculators_count, 0);
    assert_eq!(system_status_then.timers_count, 0);
    assert_eq!(system_status_then.timer_sequences_count, 0);
}

fn spawn_sequence(
    mut event_writer: EventWriter<TimerFireRequest>,
    mut should_loop: ResMut<ShouldSequenceLoop>,
    mut commands: Commands,
) {
    if should_loop.0.is_none() {
        return;
    }
    let spawn_result = if should_loop.0.take().unwrap() {
        TimerSequence::spawn_looping_sequence_and_fire_first_timer(
            &mut event_writer,
            &create_emitting_timers_vec(&mut commands),
            &mut commands,
        )
    } else {
        TimerSequence::spawn_non_looping_sequence_and_fire_first_timer(
            &mut event_writer,
            &create_emitting_timers_vec(&mut commands),
            &mut commands,
        )
    };
    if let Err(timer_sequence_error) = spawn_result {
        panic!("error spawning sequence: {:?}", timer_sequence_error)
    }
}

fn create_emitting_timers_vec(commands: &mut Commands) -> Vec<EmittingTimer> {
    let mut emitting_timers_vec = vec![];
    let empty_entity = test_dependencies::direct_spawn_empty_entity(commands);
    for _ in 0..2 {
        let redundant_calculator =
            test_dependencies::time_dependencies::direct_spawn_redundant_calculator(
                commands,
                TimerCalculatorSetPolicy::IgnoreNewIfAssigned,
            );
        let redundant_affected_entity = TimerAffectedEntity {
            affected_entity: empty_entity,
            value_calculator_entity: Some(redundant_calculator),
        };
        emitting_timers_vec.push(EmittingTimer::new(
            vec![redundant_affected_entity],
            vec![],
            TIMER_DURATION_IN_SECONDS,
            TimerDoneEventType::Nothing,
        ));
    }
    emitting_timers_vec
}

fn count_f32_going_event_value_calculators(app: &mut App) -> usize {
    app.world_mut()
        .query::<&GoingEventValueCalculator<f32>>()
        .iter(app.world())
        .len()
}

fn count_emitting_timers(app: &mut App) -> usize {
    app.world_mut()
        .query::<&EmittingTimer>()
        .iter(app.world())
        .len()
}

fn count_timer_sequences(app: &mut App) -> usize {
    app.world_mut()
        .query::<&TimerSequence>()
        .iter(app.world())
        .len()
}
