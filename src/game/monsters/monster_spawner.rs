use crate::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::Rng;
use crate::prelude::consts::Z_LAYER_MONSTER;

pub struct MonsterSpawnerPlugin;

impl Plugin for MonsterSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_initial_monsters,
                initiate_square_movement,
            )
                .chain(),
        )
            .add_systems(Update, update_monster_hearing_rings);
    }
}

pub fn spawn_initial_monsters(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    let half_window_size = WINDOW_SIZE_IN_PIXELS / 2.0;
    let third_window_size = WINDOW_SIZE_IN_PIXELS / 3.0;
    let fraction_window_size = WINDOW_SIZE_IN_PIXELS / 6.0;
    let mut rng = rand::thread_rng();
    for i in 0..INITIAL_MONSTERS_AMOUNT {
        let second_range_factor: f32 = i as f32 * third_window_size;
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Capsule2d::new(10.0, 20.0))),
                material: materials.add(Color::srgb(0.9, 0.3, 0.3)),
                transform: Transform::from_xyz(
                    rng.gen_range(-half_window_size - second_range_factor .. half_window_size + second_range_factor),
                    rng.gen_range(-half_window_size - second_range_factor .. half_window_size + second_range_factor),
                    Z_LAYER_MONSTER
                ),
                ..default()
            },
            AffectingTimerCalculators::default(),
            Monster {
                hearing_ring_distance: rng.gen_range(fraction_window_size - 35.0..fraction_window_size + 75.0),
                ..default()
            },
            WorldBoundsWrapped,
        ));
    }
}

pub fn initiate_square_movement(
    mut event_writer: EventWriter<TimerFireRequest>,
    mut commands: Commands,
    monsters_query: Query<Entity, With<Monster>>,
) {
    let fraction_window_size = WINDOW_SIZE_IN_PIXELS / 6.0;
    let mut rng = rand::thread_rng();
    for monster_entity in &monsters_query {
        let delta = rng.gen_range(fraction_window_size..150.0 + fraction_window_size);
        let is_cursed_pentagon = rng.gen::<bool>();
        if is_cursed_pentagon {
            let mut all_path_vertices = PathTravelType::Cycle.apply_to_path(vec![
                Vec3::new(0.0, 150.0, 0.0),
                Vec3::new(100.0, -150.0, 0.0),
                Vec3::new(-150.0, 50.0, 0.0),
                Vec3::new(150.0, 50.0, 0.0),
                Vec3::new(-100.0, -150.0, 0.0),
            ]);
            let is_reversed = rng.gen::<bool>();
            if is_reversed {
                all_path_vertices.reverse();
            }
            initiate_movement_along_path(
                &mut event_writer,
                monster_entity,
                rng.gen_range(1.0..3.0),
                all_path_vertices,
                &mut commands,
            );
        } else {
            let mut all_path_vertices = PathTravelType::Cycle.apply_to_path(vec![
                Vec3::new(delta, delta, 0.0),
                Vec3::new(delta, -delta, 0.0),
                Vec3::new(-delta, -delta, 0.0),
                Vec3::new(-delta, delta, 0.0),
            ]);
            let is_reversed = rng.gen::<bool>();
            if is_reversed {
                all_path_vertices.reverse();
            }
            initiate_movement_along_path(
                &mut event_writer,
                monster_entity,
                rng.gen_range(1.0..3.0),
                all_path_vertices,
                &mut commands,
            );
        }
    }
}

fn initiate_movement_along_path(
    event_writer: &mut EventWriter<TimerFireRequest>,
    monster_entity: Entity,
    timers_duration: f32,
    all_path_vertices: Vec<Vec3>,
    commands: &mut Commands,
) {
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

fn update_monster_hearing_rings(
    mut monsters_query: Query<(&Transform, &mut Monster), With<Monster>>,
    surrounding_objects_query: Query<(&Transform, Option<&Bomb>, Option<&Player>)>,
) {
    for (monster_transform, mut monster) in monsters_query.iter_mut() {
        for (surrounding_object_transform, bomb, player) in surrounding_objects_query.iter() {
            if is_point_inside_ring(surrounding_object_transform, monster_transform, monster.hearing_ring_distance) {
                if player.is_some() {
                    monster.state = MonsterState::Chasing;
                    break;
                }
                if bomb.is_some() {
                    monster.state = MonsterState::Fleeing;
                    break;
                }
            }
            monster.state = MonsterState::Idle;
        }
    }
}

fn is_point_inside_ring(point: &Transform, ring: &Transform, radius: f32) -> bool {
    let distance_x = (point.translation.x - ring.translation.x).powf(2.0);
    let distance_y = (point.translation.y - ring.translation.y).powf(2.0);
    distance_x + distance_y < radius.powf(2.0)
}