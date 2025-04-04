use bevy::{math::bounding::BoundingCircle, sprite::*};
use bevy_light_2d::light::PointLight2d;
use rand::Rng;

use crate::{prelude::*, read_no_field_variant};

pub struct BombSpawnerPlugin;

impl Plugin for BombSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_bombs).add_systems(
            Update,
            (
                (
                    listen_for_bomb_spawning_requests,
                    listen_for_bombs_done_growing,
                )
                    .in_set(TickingSystemSet::PostTicking),
                respawn_initial_bomb_on_game_restart.in_set(GameRestartSystemSet::Spawning),
            ),
        );
    }
}

fn respawn_initial_bomb_on_game_restart(
    mut event_reader: EventReader<GameEvent>,
    timer_fire_request_writer: EventWriter<TimerFireRequest>,
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    sprites_atlas_resource: ResMut<StaticImageHandles>,
    bombs_query: Query<&Bomb>,
    current_app_state: Res<State<AppState>>,
    commands: Commands,
) {
    if read_no_field_variant!(event_reader, GameEvent::RestartGame).count() > 0 {
        spawn_initial_bombs(
            timer_fire_request_writer,
            transforms_not_to_spawn_next_to,
            sprites_atlas_resource,
            bombs_query,
            current_app_state,
            commands,
        );
    }
}

fn spawn_initial_bombs(
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    mut sprites_atlas_resource: ResMut<StaticImageHandles>,
    bombs_query: Query<&Bomb>,
    current_app_state: Res<State<AppState>>,
    mut commands: Commands,
) {
    if let Err(bomb_error) = try_spawning_a_bomb(
        &mut timer_fire_request_writer,
        &transforms_not_to_spawn_next_to,
        &mut sprites_atlas_resource,
        &bombs_query,
        current_app_state.get(),
        &mut commands,
    ) {
        print_warning(bomb_error, vec![LogCategory::RequestNotFulfilled]);
    }
}

fn listen_for_bomb_spawning_requests(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    mut sprites_atlas_resource: ResMut<StaticImageHandles>,
    bombs_query: Query<&Bomb>,
    current_app_state: Res<State<AppState>>,
    mut commands: Commands,
) {
    for done_event in timer_done_event_reader.read() {
        if let TimerDoneEventType::Spawn(SpawnRequestType::Bomb) = done_event.event_type {
            if let Err(bomb_error) = try_spawning_a_bomb(
                &mut timer_fire_request_writer,
                &transforms_not_to_spawn_next_to,
                &mut sprites_atlas_resource,
                &bombs_query,
                current_app_state.get(),
                &mut commands,
            ) {
                print_warning(bomb_error, vec![LogCategory::RequestNotFulfilled]);
            }
        }
    }
}

fn try_spawning_a_bomb(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    transforms_not_to_spawn_next_to: &Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    sprites_atlas_resource: &mut ResMut<StaticImageHandles>,
    bombs_query: &Query<&Bomb>,
    current_app_state: &AppState,
    commands: &mut Commands,
) -> Result<(), BombError> {
    if bombs_query.iter().count() >= MAX_BOMB_COUNT {
        return Ok(());
    }
    let place_to_spawn_in =
        try_finding_place_for_bomb(transforms_not_to_spawn_next_to, current_app_state)?;
    let bomb_component = Bomb::default();
    let newborn_bomb = commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: bomb_component.to_colors().unwrap().bomb,
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                texture: sprites_atlas_resource.pumpkin_grey_image_handle.clone(),
                transform: Transform::from_translation(place_to_spawn_in)
                    .with_scale(Vec3::ONE * BOMB_SPAWN_SCALE),
                ..default()
            },
            AffectingTimerCalculators::default(),
            bomb_component,
            WorldBoundsWrapped,
            BombAffected::default(),
            ExplodeInContact {
                bounding_circle: BoundingCircle::new(
                    place_to_spawn_in.truncate(),
                    BOMB_SPAWN_SCALE * BOMB_SIZE,
                ),
            },
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
            TimerDoneEventType::SpawnChildForAffectedEntities(SpawnRequestType::BombText),
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
                Vec3::ONE * BOMB_SPAWN_SCALE,
                Vec3::ONE,
                Interpolator::default(),
            ),
            TimerGoingEventType::Scale,
        ))
        .id()
}

fn try_finding_place_for_bomb(
    transforms_not_to_spawn_next_to: &Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    current_app_state: &AppState,
) -> Result<Vec3, BombError> {
    if FunctionalityOverride::AlwaysSpawnBombsInMiddle.enabled() {
        return Ok(Vec3::ZERO);
    } else if let AppState::Menu = current_app_state {
        return Ok(BOMB_SPAWN_LOCATION_ON_MENU);
    }

    let mut rng = rand::thread_rng();
    let as_far_as_a_bomb_can_spawn = WINDOW_SIZE_IN_PIXELS / 2.0 - BOMB_SIZE * 2.0;
    'bomb_spawning_loop: for _attempt in 0..BOMB_SPAWNING_ATTEMPTS {
        let vector = Vec3::new(
            rng.gen_range(-as_far_as_a_bomb_can_spawn..as_far_as_a_bomb_can_spawn),
            rng.gen_range(-as_far_as_a_bomb_can_spawn..as_far_as_a_bomb_can_spawn),
            Z_LAYER_BOMB,
        );
        for transform in transforms_not_to_spawn_next_to {
            if vector.distance(transform.translation) < BOMB_SAFE_RADIUS {
                continue 'bomb_spawning_loop;
            }
        }
        return Ok(vector);
    }
    Err(BombError::CouldntFindAPlaceToSpawnBombIn)
}

fn listen_for_bombs_done_growing(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    mut commands: Commands,
    bomb_query: Query<&Bomb>,
    text_fonts_resource: ResMut<TextFonts>,
) {
    for done_event in timer_done_event_reader.read() {
        if let TimerDoneEventType::SpawnChildForAffectedEntities(SpawnRequestType::BombText) =
            done_event.event_type
        {
            for affected_entity in done_event.affected_entities.affected_entities_iter() {
                if let Ok(bomb) = bomb_query.get(affected_entity) {
                    commands.entity(affected_entity).insert(BombTag); //Ensuring no collision before fully spawned
                    commands
                        .spawn((
                            Text2dBundle {
                                text: Text::from_section(
                                    format!("{:?}", bomb.full_duration),
                                    TextStyle {
                                        font: text_fonts_resource.kenny_pixel_handle.clone(),
                                        font_size: BOMB_TIME_LEFT_FONT_SIZE,
                                        color: bomb.to_colors().unwrap().text,
                                    },
                                ),
                                transform: Transform::from_translation(Vec3::new(2.0, -2.0, 1.0)),
                                ..default()
                            },
                            PointLight2d {
                                color: bomb.to_colors().unwrap().text,
                                radius: BOMB_EXPLOSION_RADIUS,
                                intensity: BOMB_NORMAL_LIGHT_INTENSITY,
                                ..default()
                            },
                        ))
                        .set_parent(affected_entity);
                }
            }
        }
    }
}
