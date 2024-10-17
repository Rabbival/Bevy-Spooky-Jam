use std::time::Duration;

use crate::prelude::*;
use bevy::math::NormedVectorSpace;
use bevy::prelude::*;
use rand::Rng;

pub struct ExplosionManagerPlugin;

impl Plugin for ExplosionManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (listen_for_done_bombs, explode_bombs_on_direct_collision),
                manage_bomb_explosion_side_effects,
                execute_animations,
            )
                .chain()
                .in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn explode_bombs_on_direct_collision(
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
    mut bomb_exploded_event_writer: EventWriter<BombExploded>,
    explode_in_contact_query: Query<(&Transform, Option<&Monster>, Option<&Bomb>)>,
    bomb_query: Query<(&Transform, &Bomb)>,
    transform_query: Query<
        (
            &Transform,
            Entity,
            Option<&AffectingTimerCalculators>,
            Option<&Monster>,
        ),
        With<WorldBoundsWrapped>,
    >,
    mut commands: Commands,
) {
    for (bomb_transform, bomb) in &bomb_query {
        if let BombState::PostHeld = bomb.state {
            for (transform, maybe_monster, maybe_bomb) in &explode_in_contact_query {
                if bomb_transform == transform || (maybe_monster.is_none() && maybe_bomb.is_none())
                {
                    continue;
                }
                if let Some(bomb) = maybe_bomb {
                    if let BombState::PreHeld | BombState::Held = bomb.state {
                        continue;
                    }
                }
                if bomb_transform.translation.distance(transform.translation) <= BOMB_SIZE {
                    unslow_time_if_was_held(&mut time_multiplier_request_writer, bomb);
                    explode_bomb(
                        bomb_transform,
                        bomb.explosion_radius,
                        &transform_query,
                        &mut bomb_exploded_event_writer,
                        &mut timer_fire_request_writer,
                        &mut commands,
                    );
                }
            }
        }
    }
}

fn listen_for_done_bombs(
    mut timer_done_reader: EventReader<TimerDoneEvent>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
    mut bomb_exploded_event_writer: EventWriter<BombExploded>,
    bomb_query: Query<(&Transform, &Bomb)>,
    transform_query: Query<
        (
            &Transform,
            Entity,
            Option<&AffectingTimerCalculators>,
            Option<&Monster>,
        ),
        With<WorldBoundsWrapped>,
    >,
    mut commands: Commands,
) {
    for done_timer in timer_done_reader.read() {
        if let TimerDoneEventType::ExplodeInRadius(explosion_radius) = done_timer.event_type {
            for affected_entity in done_timer.affected_entities.affected_entities_iter() {
                if let Ok((bomb_transform, bomb)) = bomb_query.get(affected_entity) {
                    unslow_time_if_was_held(&mut time_multiplier_request_writer, bomb);
                    explode_bomb(
                        bomb_transform,
                        explosion_radius,
                        &transform_query,
                        &mut bomb_exploded_event_writer,
                        &mut timer_fire_request_writer,
                        &mut commands,
                    );
                } else {
                    print_error(
                        EntityError::EntityNotInQuery(
                            "bomb when explosion requested by done countdown",
                        ),
                        vec![LogCategory::RequestNotFulfilled],
                    );
                }
            }
        }
    }
}

fn unslow_time_if_was_held(
    time_multiplier_request_writer: &mut EventWriter<SetTimeMultiplier>,
    bomb: &Bomb,
) {
    if let BombState::Held = bomb.state {
        time_multiplier_request_writer.send(SetTimeMultiplier {
            multiplier_id: TimeMultiplierId::GameTimeMultiplier,
            new_multiplier: 1.0,
            duration: SLOW_MOTION_KICK_IN_AND_OUT_TIME,
        });
    }
}

fn explode_bomb(
    bomb_transform: &Transform,
    explosion_radius: f32,
    transform_query: &Query<
        (
            &Transform,
            Entity,
            Option<&AffectingTimerCalculators>,
            Option<&Monster>,
        ),
        With<WorldBoundsWrapped>,
    >,
    bomb_exploded_event_writer: &mut EventWriter<BombExploded>,
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    commands: &mut Commands,
) {
    for (transform_in_radius, entity_in_radius, maybe_affecting_timer_calculators, maybe_monster) in
        transform_query
    {
        let distance_from_bomb = bomb_transform
            .translation
            .distance(transform_in_radius.translation);
        if distance_from_bomb <= explosion_radius {
            knock_back_and_destroy(
                timer_fire_request_writer,
                bomb_transform,
                transform_in_radius,
                entity_in_radius,
                maybe_affecting_timer_calculators,
                commands,
            );
            bomb_exploded_event_writer.send(BombExploded {
                location: bomb_transform.translation,
                hit_monster: maybe_monster.is_some(),
            });
        }
    }
}

fn knock_back_and_destroy(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    bomb_transform: &Transform,
    transform_in_radius: &Transform,
    entity_in_radius: Entity,
    maybe_affecting_timer_calculators: Option<&AffectingTimerCalculators>,
    commands: &mut Commands,
) {
    let blast_move_calculator: Option<Entity> = if transform_in_radius == bomb_transform {
        None
    } else {
        Some(move_due_to_blast_calculator(
            bomb_transform,
            transform_in_radius,
            commands,
        ))
    };
    let despawn_policy = if maybe_affecting_timer_calculators.is_some() {
        DespawnPolicy::DespawnSelfAndAllThatAffectsIt
    } else {
        DespawnPolicy::DespawnSelf
    };
    timer_fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: entity_in_radius,
                value_calculator_entity: blast_move_calculator,
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            POST_BOMB_HIT_DESPAWN_TIME,
            TimerDoneEventType::DespawnAffectedEntities(despawn_policy),
        ),
        parent_sequence: None,
    });
}

fn move_due_to_blast_calculator(
    bomb_transform: &Transform,
    object_in_blast_transform: &Transform,
    commands: &mut Commands,
) -> Entity {
    let location_delta_from_bomb =
        object_in_blast_transform.translation - bomb_transform.translation;
    let blast_strength =
        BOMB_BLAST_FACTOR / location_delta_from_bomb.norm_squared().clamp(5.0, 100.0);
    let delta_due_to_blast = location_delta_from_bomb.normalize() * blast_strength;
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::AppendToTimersOfType,
            ValueByInterpolation::new(
                object_in_blast_transform.translation,
                delta_due_to_blast,
                Interpolator::new(0.8),
            ),
            TimerGoingEventType::Move(MovementType::InDirectLine),
        ))
        .id()
}

fn manage_bomb_explosion_side_effects(
    mut explosions_listener: EventReader<BombExploded>,
    mut sounds_event_writer: EventWriter<SoundEvent>,
    mut update_player_score_event_writer: EventWriter<AppendToPlayerScoreEvent>,
    bomb_explosion_sprites_atlas_resource: Res<BombExplosionSpritesAtlas>,
    sprites_atlas_resource: Res<SpritesAtlas>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    for exploded_bomb in explosions_listener.read() {
        sounds_event_writer.send(SoundEvent::BombExplodeSoundEvent);
        commands.spawn((
            SpriteBundle {
                texture: sprites_atlas_resource.floor_hole_handle.clone(),
                transform: Transform::from_xyz(
                    exploded_bomb.location.x,
                    exploded_bomb.location.y,
                    Z_LAYER_FLOOR_HOLE,
                ),
                ..default()
            },
            WorldBoundsWrapped,
        ));
        let animation_config = AnimationConfig::new(0, 60, 240);
        commands.spawn((
            SpriteBundle {
                texture: bomb_explosion_sprites_atlas_resource.image_handle.clone(),
                transform: Transform::from_xyz(
                    exploded_bomb.location.x,
                    exploded_bomb.location.y,
                    Z_LAYER_BOMB_EXPLOSION,
                )
                .with_rotation(Quat::from_rotation_z(rng.gen_range(0.0..360.0)))
                .with_scale(Vec3::new(2.5, 2.5, 0.0)),
                ..default()
            },
            TextureAtlas {
                layout: bomb_explosion_sprites_atlas_resource.atlas_handle.clone(),
                index: animation_config.first_sprite_index,
            },
            animation_config,
            WorldBoundsWrapped,
        ));
        AnimationConfig::timer_from_fps(240);

        if exploded_bomb.hit_monster {
            update_player_score_event_writer.send(AppendToPlayerScoreEvent(
                PLAYER_SCORE_POINTS_ON_MONSTER_KILLED,
            ));
        }
    }
}

#[derive(Component)]
struct AnimationConfig {
    first_sprite_index: usize,
    last_sprite_index: usize,
    fps: u8,
    frame_timer: Timer,
}

impl AnimationConfig {
    fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Transform, &mut TextureAtlas)>,
    time_multipliers: Query<&TimeMultiplier>,
) {
    for (mut config, mut transform, mut atlas) in &mut query {
        for time_multiplier in &time_multipliers {
            if let TimeMultiplierId::GameTimeMultiplier = time_multiplier.id() {
                let time_to_multiply_delta_in = time_multiplier.value();

                // we track how long the current sprite has been displayed for
                config
                    .frame_timer
                    .tick(time.delta() * time_to_multiply_delta_in as u32);

                // If it has been displayed for the user-defined amount of time (fps)...
                if config.frame_timer.just_finished() {
                    if atlas.index == config.last_sprite_index {
                        // ...and it IS the last frame, then we move back to the first frame and stop.
                        atlas.index = config.first_sprite_index;
                        // TODO remove sprite
                        transform.scale.x = 0.0;
                        transform.scale.y = 0.0;
                    } else {
                        // ...and it is NOT the last frame, then we move to the next frame...
                        atlas.index += 1;
                        // ...and reset the frame timer to start counting all over again
                        config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                        transform.scale.x += 0.04;
                        transform.scale.y += 0.04;
                    }
                }
            }
        }
    }
}
