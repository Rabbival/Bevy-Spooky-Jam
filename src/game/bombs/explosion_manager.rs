use bevy::math::NormedVectorSpace;

use crate::prelude::*;

pub struct ExplosionManagerPlugin;

impl Plugin for ExplosionManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_for_done_bombs);
    }
}

fn listen_for_done_bombs(
    mut timer_done_reader: EventReader<TimerDoneEvent>,
    mut timer_fire_request: EventWriter<TimerFireRequest>,
    bomb_query: Query<&Transform, With<Bomb>>,
    transform_query: Query<
        (&Transform, Entity, Option<&AffectingTimerCalculators>),
        With<WorldBoundsWrapped>,
    >,
    mut commands: Commands,
) {
    for done_timer in timer_done_reader.read() {
        if let TimerDoneEventType::ExplodeInRadius(explosion_radius) = done_timer.event_type {
            for affected_entity in done_timer.affected_entities.affected_entities_iter() {
                if let Ok(bomb_transform) = bomb_query.get(affected_entity) {
                    explode_bomb(
                        bomb_transform,
                        explosion_radius,
                        &transform_query,
                        &mut timer_fire_request,
                        &mut commands,
                    );
                } else {
                    print_error(
                        EntityError::EntityNotInQuery("bomb when explosion request"),
                        vec![LogCategory::RequestNotFulfilled],
                    );
                }
            }
        }
    }
}

fn explode_bomb(
    bomb_transform: &Transform,
    explosion_radius: f32,
    transform_query: &Query<
        (&Transform, Entity, Option<&AffectingTimerCalculators>),
        With<WorldBoundsWrapped>,
    >,
    timer_fire_request: &mut EventWriter<TimerFireRequest>,
    commands: &mut Commands,
) {
    for (transform, entity, maybe_affecting_timer_calculators) in transform_query {
        if transform.translation.distance(bomb_transform.translation) <= explosion_radius {
            let blast_move_calculator =
                move_due_to_blast_calculator(bomb_transform, transform, commands);
            let despawn_policy = if maybe_affecting_timer_calculators.is_some() {
                DespawnPolicy::DespawnSelfAndAffectingTimersAndParentSequences
            } else {
                DespawnPolicy::DespawnSelf
            };
            timer_fire_request.send(TimerFireRequest {
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
    object_in_blase_transform: &Transform,
    commands: &mut Commands,
) -> Entity {
    let location_delta_from_bomb =
        object_in_blase_transform.translation - bomb_transform.translation;
    let blast_strength =
        BOMB_BLAST_FACTOR / clamp_and_notify(location_delta_from_bomb.norm_squared(), 1.0, 900.0);
    let delta_due_to_blast = location_delta_from_bomb.normalize() * blast_strength;
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::AppendToTimersOfType,
            ValueByInterpolation::new(
                object_in_blase_transform.translation,
                delta_due_to_blast,
                Interpolator::new(0.8),
            ),
            TimerGoingEventType::Move(MovementType::InDirectLine),
        ))
        .id()
}
