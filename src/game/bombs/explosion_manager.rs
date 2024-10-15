use crate::prelude::*;
use bevy::math::NormedVectorSpace;

pub struct ExplosionManagerPlugin;

impl Plugin for ExplosionManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (listen_for_done_bombs, explode_bombs_on_direct_collision),
                manage_bomb_explosion_side_effects,
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
        DespawnPolicy::DespawnSelfAndAffectingTimersAndParentSequences
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
        BOMB_BLAST_FACTOR / location_delta_from_bomb.norm_squared().clamp(16.0, 2500.0);
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
    sprites_atlas_resource: Res<SpritesAtlas>,
    mut commands: Commands,
) {
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

        if exploded_bomb.hit_monster {
            sounds_event_writer.send(SoundEvent::MonsterDeathCry);
            update_player_score_event_writer.send(AppendToPlayerScoreEvent(
                PLAYER_SCORE_POINTS_ON_MONSTER_KILLED,
            ));
        }
    }
}
