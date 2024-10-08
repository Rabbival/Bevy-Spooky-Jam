use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::Rng;

use crate::prelude::*;

pub struct BombSpawnerPlugin;

impl Plugin for BombSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_inital_bombs)
            .add_systems(Update, listen_for_bomb_spawning_requests);
    }
}

fn spawn_inital_bombs(
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Monster>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if let Err(bomb_error) = try_spawning_a_bomb(
        &mut timer_fire_request_writer,
        &transforms_not_to_spawn_next_to,
        &mut meshes,
        &mut materials,
        &mut commands,
    ) {
        print_warning(bomb_error, vec![LogCategory::RequestNotFulfilled]);
    }
}

fn listen_for_bomb_spawning_requests(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Monster>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for done_event in timer_done_event_reader.read() {
        if let TimerDoneEventType::Spawn(SpawnRequestType::Bomb) = done_event.event_type {
            if let Err(bomb_error) = try_spawning_a_bomb(
                &mut timer_fire_request_writer,
                &transforms_not_to_spawn_next_to,
                &mut meshes,
                &mut materials,
                &mut commands,
            ) {
                print_warning(bomb_error, vec![LogCategory::RequestNotFulfilled]);
            }
        }
    }
}

fn try_spawning_a_bomb(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    transforms_not_to_spawn_next_to: &Query<&Transform, Or<(With<Player>, With<Monster>)>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) -> Result<(), BombError> {
    let place_to_spawn_in = try_finding_place_for_bomb(transforms_not_to_spawn_next_to)?;
    let newborn_bomb = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(BOMB_SPAWN_SIZE))),
                material: materials.add(Color::srgb(0.8, 0.2, 0.0)),
                transform: Transform::from_translation(place_to_spawn_in),
                ..default()
            },
            AffectingTimerCalculators::default(),
            Bomb::new(),
        ))
        .id();
    timer_fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: newborn_bomb,
                value_calculator_entity: Some(spawn_bomb_size_change_calculator(commands)),
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            TIME_IT_TAKES_BOMB_TO_GROW,
            TimerDoneEventType::Nothing,
        ),
        parent_sequence: None,
    });
    Ok(())
}

fn spawn_bomb_size_change_calculator(commands: &mut Commands) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::IgnoreNewIfAssigned,
            ValueByInterpolation::from_goal_and_current(
                BOMB_SPAWN_SIZE,
                BOMB_FULL_SIZE,
                Interpolator::default(),
            ),
            TimerGoingEventType::Scale,
        ))
        .id()
}

fn try_finding_place_for_bomb(
    transforms_not_to_spawn_next_to: &Query<&Transform, Or<(With<Player>, With<Monster>)>>,
) -> Result<Vec3, BombError> {
    if FunctionalityOverride::AlwaysSpawnBombsInMiddle.enabled() {
        return Ok(Vec3::ZERO);
    }

    let mut rng = rand::thread_rng();
    let as_far_as_a_bomb_can_spawn = WINDOW_SIZE_IN_PIXELS / 2.0 - BOMB_FULL_SIZE * 2.0;
    for _attempt in 0..BOMB_SPAWNING_ATTEMPTS {
        let vector = Vec3::new(
            rng.gen_range(-as_far_as_a_bomb_can_spawn..as_far_as_a_bomb_can_spawn),
            rng.gen_range(-as_far_as_a_bomb_can_spawn..as_far_as_a_bomb_can_spawn),
            Z_LAYER_BOMB,
        );
        for transform in transforms_not_to_spawn_next_to {
            if calculate_distance_including_through_screen_border(vector, transform.translation)
                .distance
                < BOMB_SAFE_RADIUS
            {
                continue;
            }
        }
        return Ok(vector);
    }
    Err(BombError::CouldntFindAPlaceToSpawnBombIn)
}
