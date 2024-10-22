use crate::{prelude::*, single_else_return};
use bevy::render::view::RenderLayers;
use rand::Rng;

pub struct MainCameraPlugin;

impl Plugin for MainCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_camera)
            .add_systems(Update, shake_camera_when_bombs_explode);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2dBundle {
            camera: Camera::default(),
            transform: Transform::from_xyz(0.0, TOP_UI_HEADER_BAR_HEIGHT / 2.0, CAMERA_Z_LAYER),
            ..default()
        },
        RenderLayers::layer(0),
        AffectingTimerCalculators::default(),
        DoNotDestroyOnRestart,
    ));
}

fn shake_camera_when_bombs_explode(
    mut event_reader: EventReader<BombExploded>,
    mut timer_fire_writer: EventWriter<TimerFireRequest>,
    player_transforms: Query<&Transform, With<Player>>,
    camera_query: Query<Entity, With<MainCamera>>,
    mut commands: Commands,
) {
    for explosion in event_reader.read() {
        let camera_entity = single_else_return!(camera_query);
        for player_transform in &player_transforms {
            let explosion_strength_to_player =
                300.0 / player_transform.translation.distance(explosion.location);
            if explosion_strength_to_player > 0.1 {
                shake_camera(
                    &mut timer_fire_writer,
                    camera_entity,
                    min(explosion_strength_to_player, 10.0),
                    &mut commands,
                );
            }
        }
    }
}

fn shake_camera(
    timer_fire_writer: &mut EventWriter<TimerFireRequest>,
    camera_entity: Entity,
    explosion_strength: f32,
    commands: &mut Commands,
) {
    let mut value_calculators = vec![];
    let mut timers = vec![];
    let shakes = generate_shake_path(explosion_strength);
    let shakes_count = shakes.len() as f32;
    for vertice in shakes {
        spawn_and_push_calculator_there_and_back(&mut value_calculators, &vertice, commands);
    }
    for calculator in value_calculators {
        timers.push(EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: camera_entity,
                value_calculator_entity: Some(calculator),
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            CAMERA_SHAKE_DURATION / shakes_count,
            TimerDoneEventType::Nothing,
        ));
    }
    if let Err(sequence_error) = TimerSequence::spawn_non_looping_sequence_and_fire_first_timer(
        timer_fire_writer,
        &timers,
        commands,
    ) {
        print_warning(sequence_error, vec![LogCategory::RequestNotFulfilled]);
    }
}

fn generate_shake_path(explosion_strength: f32) -> Vec<Vec3> {
    let mut all_path_vertices = vec![];
    let mut rng = rand::thread_rng();
    let shakes = rng.gen_range(1..5);
    for shake in 0..shakes {
        let mut x = explosion_strength * rng.gen_range(1..(shakes - shake + 1)) as f32;
        let mut y = explosion_strength * rng.gen_range(1..(shakes - shake + 1)) as f32;
        if rng.gen_bool(0.5) {
            x *= -1.0;
        }
        if rng.gen_bool(0.5) {
            y *= -1.0;
        }
        all_path_vertices.push(Vec3::new(x, y, 0.0));
    }
    all_path_vertices
}

fn spawn_and_push_calculator_there_and_back(
    value_calculators: &mut Vec<Entity>,
    shake_location: &Vec3,
    commands: &mut Commands,
) {
    value_calculators.push(
        commands
            .spawn(GoingEventValueCalculator::new(
                TimerCalculatorSetPolicy::AppendToTimersOfType,
                ValueByInterpolation::from_goal_and_current(
                    Vec3::ZERO,
                    *shake_location,
                    Interpolator::new(2.0),
                ),
                TimerGoingEventType::Move(MovementType::InDirectLine),
            ))
            .id(),
    );
    value_calculators.push(
        commands
            .spawn(GoingEventValueCalculator::new(
                TimerCalculatorSetPolicy::AppendToTimersOfType,
                ValueByInterpolation::from_goal_and_current(
                    *shake_location,
                    Vec3::ZERO,
                    Interpolator::new(2.0),
                ),
                TimerGoingEventType::Move(MovementType::InDirectLine),
            ))
            .id(),
    );
}
