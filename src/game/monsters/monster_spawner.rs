use crate::{prelude::*, read_no_field_variant};
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use rand::Rng;

pub struct MonsterSpawnerPlugin;

impl Plugin for MonsterSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_monster).add_systems(
            Update,
            respawn_monsters_on_game_restart.in_set(GameRestartSystemSet::Respawning),
        );
        if FunctionalityOverride::SpawnOnlyOneEnemy.disabled() {
            app.add_systems(Update, listen_for_monster_spawning_requests);
        }
    }
}

fn respawn_monsters_on_game_restart(
    mut event_reader: EventReader<GameEvent>,
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    sprites_atlas_resource: ResMut<SpritesAtlas>,
    event_writer: EventWriter<TimerFireRequest>,
    commands: Commands,
) {
    for _restart_event in read_no_field_variant!(event_reader, GameEvent::RestartGame) {
        spawn_initial_monster(
            transforms_not_to_spawn_next_to,
            meshes,
            materials,
            sprites_atlas_resource,
            event_writer,
            commands,
        );
        break;
    }
}

fn spawn_initial_monster(
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut sprites_atlas_resource: ResMut<SpritesAtlas>,
    mut event_writer: EventWriter<TimerFireRequest>,
    mut commands: Commands,
) {
    if let Err(monster_error) = try_spawning_a_monster(
        &transforms_not_to_spawn_next_to,
        &mut meshes,
        &mut materials,
        &mut sprites_atlas_resource,
        &mut event_writer,
        &mut commands,
    ) {
        print_warning(monster_error, vec![LogCategory::RequestNotFulfilled]);
    }
}

fn listen_for_monster_spawning_requests(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut sprites_atlas_resource: ResMut<SpritesAtlas>,
    mut event_writer: EventWriter<TimerFireRequest>,
    mut commands: Commands,
) {
    for done_event in timer_done_event_reader.read() {
        if let TimerDoneEventType::Spawn(SpawnRequestType::Monster) = done_event.event_type {
            if let Err(monster_error) = try_spawning_a_monster(
                &transforms_not_to_spawn_next_to,
                &mut meshes,
                &mut materials,
                &mut sprites_atlas_resource,
                &mut event_writer,
                &mut commands,
            ) {
                print_warning(monster_error, vec![LogCategory::RequestNotFulfilled]);
            }
        }
    }
}

fn try_spawning_a_monster(
    transforms_not_to_spawn_next_to: &Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    sprites_atlas_resource: &mut ResMut<SpritesAtlas>,
    event_writer: &mut EventWriter<TimerFireRequest>,
    commands: &mut Commands,
) -> Result<(), MonsterError> {
    let mut rng = rand::thread_rng();
    let fraction_window_size = WINDOW_SIZE_IN_PIXELS / 6.0;
    let place_to_spawn_in = try_finding_place_for_monster(transforms_not_to_spawn_next_to)?;
    let monster_entity = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(80.0, 50.0))),
                material: materials.add(ColorMaterial {
                    color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                    texture: Some(sprites_atlas_resource.bato_san_image_handle.clone()),
                    ..default()
                }),
                transform: Transform::from_translation(place_to_spawn_in),
                ..default()
            },
            /*TextureAtlas {
                layout: sprites_atlas_resource.atlas_handle.clone(),
                index: 0,
            },*/
            AffectingTimerCalculators::default(),
            WorldBoundsWrapped,
        ))
        .id();
    if FunctionalityOverride::EnemiesDontMove.disabled() {
        let sequence_id = spawn_path_timer_sequence(
            monster_entity,
            rng.gen_range(1.0..3.0),
            generate_initial_path_to_follow(),
            commands,
        )?;
        commands.entity(monster_entity).insert(Monster {
            hearing_ring_distance: rng
                .gen_range(fraction_window_size - 35.0..fraction_window_size + 75.0),
            state: MonsterState::Spawning,
            path_timer_sequence: sequence_id,
        });
    }
    spawn_grace_period_timer(monster_entity, event_writer, commands);
    Ok(())
}

fn spawn_grace_period_timer(
    newborn_monster: Entity,
    event_writer: &mut EventWriter<TimerFireRequest>,
    commands: &mut Commands,
) {
    let alpha_change_calculator = spawn_alpha_change_calculator(commands);
    event_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: newborn_monster,
                value_calculator_entity: Some(alpha_change_calculator),
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            TIME_IT_TAKES_MONSTERS_TO_SPAWN,
            TimerDoneEventType::DeclareSpawnDone,
        ),
        parent_sequence: None,
    });
}

fn spawn_alpha_change_calculator(commands: &mut Commands) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::IgnoreNewIfAssigned,
            ValueByInterpolation::from_goal_and_current(
                0.0,
                MONSTER_FADED_ALPHA,
                Interpolator::default(),
            ),
            TimerGoingEventType::SetAlpha,
        ))
        .id()
}

fn try_finding_place_for_monster(
    transforms_not_to_spawn_next_to: &Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
) -> Result<Vec3, MonsterError> {
    let mut rng = rand::thread_rng();
    let as_far_as_a_monster_can_spawn = WINDOW_SIZE_IN_PIXELS / 2.0 - MONSTER_FULL_SIZE * 2.0;
    'monster_spawning_loop: for _attempt in 0..MONSTER_SPAWNING_ATTEMPTS {
        let vector = Vec3::new(
            rng.gen_range(-as_far_as_a_monster_can_spawn..as_far_as_a_monster_can_spawn),
            rng.gen_range(-as_far_as_a_monster_can_spawn..as_far_as_a_monster_can_spawn),
            Z_LAYER_MONSTER,
        );
        for transform in transforms_not_to_spawn_next_to {
            if vector.distance(transform.translation) < MONSTER_SAFE_RADIUS {
                continue 'monster_spawning_loop;
            }
        }
        return Ok(vector);
    }
    Err(MonsterError::CouldntFindAPlaceToSpawnMonsterIn)
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
    if emitting_timers.is_empty() {
        Err(TimerSequenceError::TriedToFireATimerSequenceWithNoTimers)
    } else {
        Ok(commands
            .spawn(TimerSequence::looping_sequence(&emitting_timers))
            .id())
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
