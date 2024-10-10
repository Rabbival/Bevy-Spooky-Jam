use bevy::math::NormedVectorSpace;

use crate::prelude::*;

pub struct ExplosionManagerPlugin;

impl Plugin for ExplosionManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (listen_for_done_bombs, explode_bombs_colliding_with_monsters)
                .in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn explode_bombs_colliding_with_monsters(
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
    monster_query: Query<&Transform, With<Monster>>,
    bomb_query: Query<(&Transform, &Bomb)>,
    transform_query: Query<
        (&Transform, Entity, Option<&AffectingTimerCalculators>),
        With<WorldBoundsWrapped>,
    >,
    mut commands: Commands,
) {
    for (bomb_transform, bomb) in &bomb_query {
        if let BombState::PostHeld = bomb.bomb_state {
            for monster_transform in &monster_query {
                if bomb_transform
                    .translation
                    .distance(monster_transform.translation)
                    <= BOMB_SIZE
                {
                    unslow_time_if_was_held(&mut time_multiplier_request_writer, bomb);
                    explode_bomb(
                        bomb_transform,
                        bomb.explosion_radius,
                        &transform_query,
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
    bomb_query: Query<(&Transform, &Bomb)>,
    transform_query: Query<
        (&Transform, Entity, Option<&AffectingTimerCalculators>),
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
    if let BombState::Held = bomb.bomb_state {
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
        (&Transform, Entity, Option<&AffectingTimerCalculators>),
        With<WorldBoundsWrapped>,
    >,
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    commands: &mut Commands,
) {
    for (transform, entity, maybe_affecting_timer_calculators) in transform_query {
        let distance_from_bomb = bomb_transform.translation.distance(transform.translation);
        if distance_from_bomb <= explosion_radius {
            let blast_move_calculator =
                move_due_to_blast_calculator(bomb_transform, transform, commands);
            let despawn_policy = if maybe_affecting_timer_calculators.is_some() {
                DespawnPolicy::DespawnSelfAndAffectingTimersAndParentSequences
            } else {
                DespawnPolicy::DespawnSelf
            };
            timer_fire_request_writer.send(TimerFireRequest {
                timer: EmittingTimer::new(
                    vec![TimerAffectedEntity {
                        affected_entity: entity,
                        value_calculator_entity: Some(blast_move_calculator),
                    }],
                    vec![TimeMultiplierId::GameTimeMultiplier],
                    POST_BOMB_HIT_DESPAWN_TIME,
                    TimerDoneEventType::DespawnAffectedEntities(despawn_policy),
                ),
                parent_sequence: None,
            });
        }
    }
}

fn move_due_to_blast_calculator(
    bomb_transform: &Transform,
    object_in_blast_transform: &Transform,
    commands: &mut Commands,
) -> Entity {
    let location_delta_from_bomb =
        object_in_blast_transform.translation - bomb_transform.translation;
    let blast_strength =
        BOMB_BLAST_FACTOR / clamp_and_notify(location_delta_from_bomb.norm_squared(), 4.0, 2500.0);
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
