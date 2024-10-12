use bevy::sprite::*;
use rand::Rng;

use crate::prelude::*;

pub struct BombSpawnerPlugin;

impl Plugin for BombSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_initial_bombs).add_systems(
            Update,
            (
                listen_for_bomb_spawning_requests,
                listen_for_bombs_done_growing,
            ),
        );
    }
}

fn spawn_initial_bombs(
    mut sprites_atlas_resource: ResMut<SpritesAtlas>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    if let Err(bomb_error) = try_spawning_a_bomb(
        &mut sprites_atlas_resource,
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
    mut sprites_atlas_resource: ResMut<SpritesAtlas>,
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    transforms_not_to_spawn_next_to: Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for done_event in timer_done_event_reader.read() {
        if let TimerDoneEventType::Spawn(SpawnRequestType::Bomb) = done_event.event_type {
            if let Err(bomb_error) = try_spawning_a_bomb(
                &mut sprites_atlas_resource,
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
    sprites_atlas_resource: &mut ResMut<SpritesAtlas>,
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    transforms_not_to_spawn_next_to: &Query<&Transform, Or<(With<Player>, With<Bomb>)>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    commands: &mut Commands,
) -> Result<(), BombError> {
    let place_to_spawn_in = try_finding_place_for_bomb(transforms_not_to_spawn_next_to)?;
    let newborn_bomb = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(BOMB_SIZE))),
                material: materials.add(ColorMaterial {
                    color: BombState::PreHeld.to_colors().unwrap().bomb,
                    texture: Some(sprites_atlas_resource.pumpkin_image_handle.clone()),
                    ..default()
                }),
                transform: Transform::from_translation(place_to_spawn_in)
                    .with_scale(Vec3::ONE * BOMB_SPAWN_SCALE),
                ..default()
            },
            TextureAtlas {
                layout: sprites_atlas_resource.atlas_handle.clone(),
                index: 0,
            },
            AffectingTimerCalculators::default(),
            Bomb::default(),
            WorldBoundsWrapped,
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
) -> Result<Vec3, BombError> {
    if FunctionalityOverride::AlwaysSpawnBombsInMiddle.enabled() {
        return Ok(Vec3::ZERO);
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
                    commands
                        .spawn(Text2dBundle {
                            text: Text::from_section(
                                format!("{:?}", bomb.full_duration),
                                TextStyle {
                                    font: text_fonts_resource.kenny_pixel_handle.clone(),
                                    font_size: BOMB_TIME_LEFT_FONT_SIZE,
                                    color: BombState::PreHeld.to_colors().unwrap().text,
                                },
                            ),
                            transform: Transform::from_translation(Vec3::Z),
                            ..default()
                        })
                        .set_parent(affected_entity);
                }
            }
        }
    }
}
