use rand::Rng;

use crate::prelude::*;

pub struct MonsterMainPathInitiationPlugin;

impl Plugin for MonsterMainPathInitiationPlugin {
    fn build(&self, app: &mut App) {
        if FunctionalityOverride::EnemiesDontMove.disabled() {
            app.add_systems(
                Update,
                listen_for_spawn_phase_ending.in_set(MonsterSystemSet::PathAndVisualUpdating),
            );
        }
    }
}

fn listen_for_spawn_phase_ending(
    mut monster_state_change_event: EventReader<MonsterStateChanged>,
    mut timer_fire_writer: EventWriter<TimerFireRequest>,
    mut monsters_query: Query<&mut Monster>,
    mut commands: Commands,
) {
    for event in monster_state_change_event.read() {
        if let MonsterState::Spawning = event.previous_state {
            if event.next_state == MonsterState::default() {
                if let Ok(mut monster) = monsters_query.get_mut(event.monster) {
                    let mut rng = rand::thread_rng();
                    match spawn_path_timer_sequence(
                        &mut timer_fire_writer,
                        event.monster,
                        rng.gen_range(3.0..4.5),
                        generate_initial_path_to_follow(),
                        &mut commands,
                    ) {
                        Ok(timer_sequence_entity) => {
                            monster.path_timer_sequence = Some(timer_sequence_entity);
                        }
                        Err(timer_sequence_error) => {
                            print_error(
                                timer_sequence_error,
                                vec![LogCategory::RequestNotFulfilled, LogCategory::Time],
                            );
                        }
                    }
                }
            }
        }
    }
}

fn generate_initial_path_to_follow() -> Vec<Vec3> {
    let mut all_path_vertices: Vec<Vec3>;
    let fraction_window_size = WINDOW_SIZE_IN_PIXELS / 6.0;
    let mut rng = rand::thread_rng();
    let is_cursed_pentagon = rng.gen::<bool>();
    if is_cursed_pentagon {
        all_path_vertices = PathTravelType::Cycle.apply_to_path(vec![
            Vec3::new(0.0, 150.0, 0.0),
            Vec3::new(100.0, -150.0, 0.0),
            Vec3::new(-150.0, 50.0, 0.0),
            Vec3::new(150.0, 50.0, 0.0),
            Vec3::new(-100.0, -150.0, 0.0),
        ]);
    } else {
        let delta = rng.gen_range(fraction_window_size..150.0 + fraction_window_size);
        all_path_vertices = PathTravelType::Cycle.apply_to_path(vec![
            Vec3::new(delta, delta, 0.0),
            Vec3::new(delta, -delta, 0.0),
            Vec3::new(-delta, -delta, 0.0),
            Vec3::new(-delta, delta, 0.0),
        ]);
    }
    let is_reversed = rng.gen::<bool>();
    if is_reversed {
        all_path_vertices.reverse();
    }
    all_path_vertices
}

fn spawn_path_timer_sequence(
    timer_fire_writer: &mut EventWriter<TimerFireRequest>,
    monster_entity: Entity,
    timers_duration: f32,
    all_path_vertices: Vec<Vec3>,
    commands: &mut Commands,
) -> Result<Entity, TimerSequenceError> {
    let going_event_value_calculators =
        configure_value_calculators_for_patroller(all_path_vertices, 2.0);
    let mut emitting_timers = vec![];
    for value_calculator in going_event_value_calculators {
        spawn_calculator_and_push_timer(
            monster_entity,
            value_calculator,
            timers_duration,
            &mut emitting_timers,
            commands,
        );
    }
    TimerSequence::spawn_looping_sequence_and_fire_first_timer(
        timer_fire_writer,
        &emitting_timers,
        commands,
    )
}

fn configure_value_calculators_for_patroller(
    all_path_vertices: Vec<Vec3>,
    interpolator_power: f32,
) -> Vec<GoingEventValueCalculator<Vec3>> {
    let mut value_calculators = vec![];
    let vertice_count = all_path_vertices.iter().len();
    for (index, vertice) in all_path_vertices.iter().enumerate() {
        if index == vertice_count - 1 {
            break;
        }
        value_calculators.push(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::AppendToTimersOfType,
            ValueByInterpolation::from_goal_and_current(
                *vertice,
                *all_path_vertices
                    .get(index + 1)
                    .unwrap_or(all_path_vertices.first().unwrap()), //if it's empty we wouldn't get in the for loop
                Interpolator::new(interpolator_power),
            ),
            TimerGoingEventType::Move(MovementType::InDirectLine),
        ));
    }
    value_calculators
}

fn spawn_calculator_and_push_timer(
    monster_entity: Entity,
    value_calculator: GoingEventValueCalculator<Vec3>,
    timer_duration: f32,
    emitting_timers: &mut Vec<EmittingTimer>,
    commands: &mut Commands,
) {
    let value_calculator_id = commands.spawn(value_calculator).id();
    emitting_timers.push(EmittingTimer::new(
        vec![TimerAffectedEntity {
            affected_entity: monster_entity,
            value_calculator_entity: Some(value_calculator_id),
        }],
        vec![TimeMultiplierId::GameTimeMultiplier],
        timer_duration,
        TimerDoneEventType::Nothing,
    ));
}
