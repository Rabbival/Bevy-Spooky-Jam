use crate::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

pub struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_initial_monsters,
                (initiate_square_movement, initiate_diagonal_movement),
            )
                .chain(),
        );
    }
}

pub fn spawn_initial_monsters(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Capsule2d::new(10.0, 20.0))),
            material: materials.add(Color::srgb(0.9, 0.3, 0.3)),
            transform: Transform::from_xyz(250.0, 250.0, 10.0),
            ..default()
        },
        AffectingTimerCalculators::default(),
        Monster,
        WorldBoundsWrapped,
    ));
}

pub fn initiate_square_movement(
    mut event_writer: EventWriter<TimerFireRequest>,
    patroller_query: Query<Entity, With<Patroller>>,
    mut commands: Commands,
) {
    for patroller_entity in &patroller_query {
        let all_path_vertices = PathTravelType::Cycle.apply_to_path(vec![
            Vec3::new(100.0, 100.0, 0.0),
            Vec3::new(100.0, -100.0, 0.0),
            Vec3::new(-100.0, -100.0, 0.0),
            Vec3::new(-100.0, 100.0, 0.0),
        ]);
        initiate_movement_along_path(
            &mut event_writer,
            patroller_entity,
            EXAMPLE_PATROLLER_SQUARE_DURATION,
            all_path_vertices,
            &mut commands,
        );
    }
}

pub fn initiate_diagonal_movement(
    mut event_writer: EventWriter<TimerFireRequest>,
    patroller_query: Query<Entity, With<Patroller>>,
    mut commands: Commands,
) {
    for patroller_entity in &patroller_query {
        let all_path_vertices = PathTravelType::Cycle.apply_to_path(vec![
            Vec3::new(150.0, 150.0, 0.0),
            Vec3::new(-150.0, -150.0, 0.0),
        ]);
        initiate_movement_along_path(
            &mut event_writer,
            patroller_entity,
            EXAMPLE_PATROLLER_DIAGON_DURATION,
            all_path_vertices,
            &mut commands,
        );
    }
}

fn initiate_movement_along_path(
    event_writer: &mut EventWriter<TimerFireRequest>,
    patroller_entity: Entity,
    timers_duration: f32,
    all_path_vertices: Vec<Vec3>,
    commands: &mut Commands,
) {
    let going_event_value_calculators =
        configure_value_calculators_for_patroller(all_path_vertices, 2.0);
    let mut emitting_timers = vec![];
    for value_calculator in going_event_value_calculators {
        spawn_calculator_and_push_timer(
            patroller_entity,
            value_calculator,
            timers_duration,
            &mut emitting_timers,
            commands,
        );
    }
    if let Err(timer_sequence_error) = TimerSequence::spawn_looping_sequence_and_fire_first_timer(
        event_writer,
        &emitting_timers,
        commands,
    ) {
        print_error(
            timer_sequence_error,
            vec![LogCategory::Time, LogCategory::RequestNotFulfilled],
        );
    }
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
    patroller_entity: Entity,
    value_calculator: GoingEventValueCalculator<Vec3>,
    timer_duration: f32,
    emitting_timers: &mut Vec<EmittingTimer>,
    commands: &mut Commands,
) {
    let value_calculator_id = commands.spawn(value_calculator).id();
    emitting_timers.push(EmittingTimer::new(
        vec![TimerAffectedEntity {
            affected_entity: patroller_entity,
            value_calculator_entity: Some(value_calculator_id),
        }],
        vec![TimeMultiplierId::GameTimeMultiplier],
        timer_duration,
        TimerDoneEventType::Nothing,
    ));
}
