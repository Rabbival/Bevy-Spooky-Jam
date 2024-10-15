use crate::{prelude::*, read_no_field_variant};
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
    sprites_atlas_resource: ResMut<SpritesAtlas>,
    event_writer: EventWriter<TimerFireRequest>,
    commands: Commands,
) {
    for _restart_event in read_no_field_variant!(event_reader, GameEvent::RestartGame) {
        spawn_initial_monster(
            transforms_not_to_spawn_next_to,
            sprites_atlas_resource,
            event_writer,
            commands,
        );
        break;
    }
}

fn spawn_initial_monster(
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    mut sprites_atlas_resource: ResMut<SpritesAtlas>,
    mut event_writer: EventWriter<TimerFireRequest>,
    mut commands: Commands,
) {
    if let Err(monster_error) = try_spawning_a_monster(
        &transforms_not_to_spawn_next_to,
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
) -> Result<(), MonsterError> {
    let mut rng = rand::thread_rng();
    let fraction_window_size = WINDOW_SIZE_IN_PIXELS / 6.0;
    let place_to_spawn_in = try_finding_place_for_monster(transforms_not_to_spawn_next_to)?;
    let monster_entity = commands
        .spawn((
            Monster {
                hearing_ring_distance: rng
                    .gen_range(fraction_window_size - 15.0..fraction_window_size + 75.0),
                state: MonsterState::Spawning,
                ..default()
            },
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
                index: 0,
            },
            AffectingTimerCalculators::default(),
            WorldBoundsWrapped,
            PlayerMonsterCollider::new(MONSTER_COLLIDER_RADIUS),
        ))
        .id();
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
