use crate::{prelude::*, read_no_field_variant};
use rand::{rngs::ThreadRng, Rng};

pub struct MonsterSpawnerPlugin;

impl Plugin for MonsterSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_monster).add_systems(
            Update,
            respawn_monsters_on_game_restart.in_set(GameRestartSystemSet::Spawning),
        );
        if FunctionalityOverride::SpawnOnlyOneEnemy.disabled() {
            app.add_systems(Update, listen_for_monster_spawning_requests);
        }
    }
}

fn respawn_monsters_on_game_restart(
    mut event_reader: EventReader<GameEvent>,
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    sprites_atlas_resource: ResMut<SpritesAtlas>,
    event_writer: EventWriter<TimerFireRequest>,
    commands: Commands,
) {
    if read_no_field_variant!(event_reader, GameEvent::RestartGame).count() > 0 {
        spawn_initial_monster(
            transforms_not_to_spawn_next_to,
            sprites_atlas_resource,
            event_writer,
            commands,
        );
    }
}

fn spawn_initial_monster(
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    mut sprites_atlas_resource: ResMut<SpritesAtlas>,
    mut event_writer: EventWriter<TimerFireRequest>,
    mut commands: Commands,
) {
    let inital_spawn_spot = Vec3::new(0.0, WINDOW_SIZE_IN_PIXELS * 3.0 / 8.0, Z_LAYER_MONSTER);
    if let Err(monster_error) = try_spawning_a_monster(
        &transforms_not_to_spawn_next_to,
        &mut sprites_atlas_resource,
        &mut event_writer,
        &mut commands,
        Some(inital_spawn_spot),
    ) {
        print_warning(monster_error, vec![LogCategory::RequestNotFulfilled]);
    }
}

fn listen_for_monster_spawning_requests(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    mut sprites_atlas_resource: ResMut<SpritesAtlas>,
    mut event_writer: EventWriter<TimerFireRequest>,
    mut commands: Commands,
) {
    for done_event in timer_done_event_reader.read() {
        if let TimerDoneEventType::Spawn(SpawnRequestType::Monster) = done_event.event_type {
            if let Err(monster_error) = try_spawning_a_monster(
                &transforms_not_to_spawn_next_to,
                &mut sprites_atlas_resource,
                &mut event_writer,
                &mut commands,
                None,
            ) {
                print_warning(monster_error, vec![LogCategory::RequestNotFulfilled]);
            }
        }
    }
}

fn try_spawning_a_monster(
    transforms_not_to_spawn_next_to: &Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    sprites_atlas_resource: &mut ResMut<SpritesAtlas>,
    event_writer: &mut EventWriter<TimerFireRequest>,
    commands: &mut Commands,
    override_spawning_spot: Option<Vec3>,
) -> Result<(), MonsterError> {
    let mut rng = rand::thread_rng();
    let place_to_spawn_in = override_spawning_spot.unwrap_or(try_finding_place_for_monster(
        transforms_not_to_spawn_next_to,
    )?);
    let monster_component = Monster {
        hearing_ring_distance: rng.gen_range(
            BOMB_EXPLOSION_RADIUS + MONSTER_FULL_SIZE
                ..(BOMB_EXPLOSION_RADIUS + MONSTER_FULL_SIZE) * 2.0,
        ),
        state: MonsterState::Spawning,
        main_path: VecBasedArray::new(generate_initial_path_to_follow()),
        path_timer_sequence: None,
        animation_timer_sequence: None,
    };
    let monster_entity = commands.spawn((
        monster_component,
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(1.0, 1.0, 1.0, 0.0),
                custom_size: Some(Vec2::new(80.0, 50.0)),
                ..default()
            },
            texture: sprites_atlas_resource.image_handle.clone(),
            transform: Transform::from_translation(place_to_spawn_in),
            ..default()
        },
        TextureAtlas {
            layout: sprites_atlas_resource.atlas_handle.clone(),
            index: monster_component
                .heading_direction_by_index(0)
                .to_monster_initial_frame_index(),
        },
        AffectingTimerCalculators::default(),
        WorldBoundsWrapped,
    ));
    spawn_grace_period_timer(monster_entity.id(), event_writer, commands);
    Ok(())
}

fn generate_initial_path_to_follow() -> Vec<Vec3> {
    let mut all_path_vertices: Vec<Vec3>;
    let mut rng = rand::thread_rng();
    let path_code = rng.gen_range(0..3);
    all_path_vertices = get_path_by_chosen_code(path_code, &mut rng);
    let is_reversed = rng.gen::<bool>();
    if is_reversed {
        all_path_vertices.reverse();
    }
    all_path_vertices
}

fn get_path_by_chosen_code(code: usize, rng: &mut ThreadRng) -> Vec<Vec3> {
    let fraction_window_size = WINDOW_SIZE_IN_PIXELS / 6.0;
    let delta = rng.gen_range(fraction_window_size..150.0 + fraction_window_size);
    let delta2 = rng.gen_range(fraction_window_size..150.0 + fraction_window_size);
    match code {
        0 => PathTravelType::Cycle.apply_to_path(vec![
            Vec3::new(0.0, delta, 0.0),
            Vec3::new(delta2, -delta, 0.0),
            Vec3::new(-delta, delta2 / 2.0, 0.0),
            Vec3::new(delta, delta2 / 2.0, 0.0),
            Vec3::new(-delta2, -delta, 0.0),
        ]),
        1 => PathTravelType::Cycle.apply_to_path(vec![
            Vec3::new(delta, delta, 0.0),
            Vec3::new(delta, -delta, 0.0),
            Vec3::new(-delta, -delta, 0.0),
            Vec3::new(-delta, delta, 0.0),
        ]),
        _ => PathTravelType::GoBackAlongPath.apply_to_path(vec![
            Vec3::new(-delta2, delta, 0.0),
            Vec3::new(delta2, -delta, 0.0),
            Vec3::new(-delta, -delta2, 0.0),
        ]),
    }
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
