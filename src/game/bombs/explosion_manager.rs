use crate::prelude::*;
use bevy::color::palettes::css::DARK_GRAY;
use bevy::math::NormedVectorSpace;
use bevy::prelude::*;
use bevy_light_2d::light::PointLight2d;

pub struct ExplosionManagerPlugin;

impl Plugin for ExplosionManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (listen_for_done_bombs, explode_bombs_on_direct_collision),
                mark_bombs_in_explosion_as_exploded,
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
    explode_in_contact_query: Query<(&Transform, Option<&Monster>, Option<&BombTag>, Entity)>,
    mut bomb_query: Query<(&GlobalTransform, &mut Bomb, Entity)>,
    mut transform_query: Query<
        (
            &Transform,
            Entity,
            &mut BombAffected,
            Option<&AffectingTimerCalculators>,
            Option<&Monster>,
            Option<&Player>,
            Option<&BombTag>,
        ),
        With<WorldBoundsWrapped>,
    >,
    mut commands: Commands,
) {
    for (bomb_transform, mut bomb, bomb_entity) in &mut bomb_query {
        if let BombState::PostHeld = bomb.state {
            for (transform, maybe_monster, maybe_bomb, entity) in &explode_in_contact_query {
                if bomb_entity == entity || (maybe_monster.is_none() && maybe_bomb.is_none()) {
                    continue;
                }
                let radius_of_exploded = if let Some(monster) = maybe_monster {
                    if let MonsterState::Chasing(_) = monster.state {
                        MONSTER_COLLIDER_RADIUS + MONSTER_COLLIDER_RADIUS_FACTOR_WHEN_CHASING
                    } else {
                        MONSTER_COLLIDER_RADIUS
                    }
                } else {
                    BOMB_SIZE
                };
                if bomb_transform.translation().distance(transform.translation)
                    <= BOMB_SIZE + radius_of_exploded
                {
                    unslow_time_if_was_held(&mut time_multiplier_request_writer, &bomb);
                    bomb.state = BombState::Exploded;
                    explode_bomb(
                        bomb_entity,
                        bomb_transform,
                        bomb.explosion_radius,
                        &mut transform_query,
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
    mut bomb_query: Query<(&GlobalTransform, &mut Bomb, Entity)>,
    mut transform_query: Query<
        (
            &Transform,
            Entity,
            &mut BombAffected,
            Option<&AffectingTimerCalculators>,
            Option<&Monster>,
            Option<&Player>,
            Option<&BombTag>,
        ),
        With<WorldBoundsWrapped>,
    >,
    mut commands: Commands,
) {
    for done_timer in timer_done_reader.read() {
        if let TimerDoneEventType::ExplodeInRadius(explosion_radius) = done_timer.event_type {
            for affected_entity in done_timer.affected_entities.affected_entities_iter() {
                if let Ok((bomb_transform, mut bomb, bomb_entity)) =
                    bomb_query.get_mut(affected_entity)
                {
                    unslow_time_if_was_held(&mut time_multiplier_request_writer, &bomb);
                    explode_bomb(
                        bomb_entity,
                        bomb_transform,
                        explosion_radius,
                        &mut transform_query,
                        &mut bomb_exploded_event_writer,
                        &mut timer_fire_request_writer,
                        &mut commands,
                    );
                    bomb.state = BombState::Exploded;
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
    exploding_bomb_entity: Entity,
    bomb_transform: &GlobalTransform,
    explosion_radius: f32,
    transform_query: &mut Query<
        (
            &Transform,
            Entity,
            &mut BombAffected,
            Option<&AffectingTimerCalculators>,
            Option<&Monster>,
            Option<&Player>,
            Option<&BombTag>,
        ),
        With<WorldBoundsWrapped>,
    >,
    bomb_exploded_event_writer: &mut EventWriter<BombExploded>,
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    commands: &mut Commands,
) {
    let mut monsters_in_explosion = 0;
    let mut player_caught_in_explosion = false;
    for (
        transform_in_radius,
        entity_in_radius,
        mut bomb_affected_component,
        maybe_affecting_timer_calculators,
        maybe_monster,
        maybe_player,
        maybe_bomb,
    ) in transform_query
    {
        if bomb_affected_component.currently_affected_by_bomb {
            if entity_in_radius == exploding_bomb_entity {
                commands.entity(entity_in_radius).despawn_recursive();
            }
            continue;
        }
        let distance_from_bomb = bomb_transform
            .translation()
            .truncate()
            .distance(transform_in_radius.translation.truncate());
        if distance_from_bomb <= explosion_radius {
            bomb_affected_component.currently_affected_by_bomb = true;
            let done_event_type = determine_done_event_type(
                entity_in_radius == exploding_bomb_entity,
                maybe_bomb,
                maybe_affecting_timer_calculators,
            );
            knock_back_and_destroy(
                timer_fire_request_writer,
                exploding_bomb_entity,
                done_event_type,
                bomb_transform.translation(),
                transform_in_radius,
                entity_in_radius,
                commands,
            );
            if maybe_monster.is_some() {
                monsters_in_explosion += 1;
            }
            if maybe_player.is_some() {
                player_caught_in_explosion = true;
            }
        }
    }
    bomb_exploded_event_writer.send(BombExploded {
        location: bomb_transform.translation(),
        monster_hit_count: monsters_in_explosion,
        hit_player: player_caught_in_explosion,
    });
}

fn determine_done_event_type(
    is_self: bool,
    maybe_bomb: Option<&BombTag>,
    maybe_affecting_timer_calculators: Option<&AffectingTimerCalculators>,
) -> TimerDoneEventType {
    if maybe_bomb.is_some() && !is_self {
        TimerDoneEventType::ExplodeInRadius(BOMB_EXPLOSION_RADIUS)
    } else {
        let despawn_policy = if maybe_affecting_timer_calculators.is_some() {
            DespawnPolicy::DespawnSelfAndAllThatAffectsIt
        } else {
            DespawnPolicy::DespawnSelf
        };
        TimerDoneEventType::DespawnAffectedEntities(despawn_policy)
    }
}

fn knock_back_and_destroy(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    exploding_bomb_entity: Entity,
    done_event_type: TimerDoneEventType,
    exploding_bomb_location: Vec3,
    transform_in_radius: &Transform,
    entity_in_radius: Entity,
    commands: &mut Commands,
) {
    let blast_move_calculator: Option<Entity> = if entity_in_radius == exploding_bomb_entity {
        None
    } else {
        Some(move_due_to_blast_calculator(
            exploding_bomb_location,
            transform_in_radius,
            commands,
        ))
    };
    timer_fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: entity_in_radius,
                value_calculator_entity: blast_move_calculator,
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            POST_BOMB_HIT_DESPAWN_TIME,
            done_event_type,
        ),
        parent_sequence: None,
    });
}

fn move_due_to_blast_calculator(
    exploding_bomb_location: Vec3,
    object_in_blast_transform: &Transform,
    commands: &mut Commands,
) -> Entity {
    let location_delta_from_bomb = object_in_blast_transform.translation - exploding_bomb_location;
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

fn mark_bombs_in_explosion_as_exploded(
    mut explosions_listener: EventReader<BombExploded>,
    mut bomb_query: Query<(&Transform, &mut Bomb, &mut Sprite, Entity)>,
    mut text_query: Query<(&mut Text, &mut PointLight2d, &Parent)>,
) {
    for explosion in explosions_listener.read() {
        for (bomb_transform, mut bomb, mut bomb_sprite, bomb_entity) in &mut bomb_query {
            if explosion.location.distance(bomb_transform.translation) <= BOMB_EXPLOSION_RADIUS {
                if let BombState::Exploded = bomb.state {
                    continue;
                }
                bomb.state = BombState::Exploded;
                bomb.about_to_explode = true;
                for (mut text, mut light, text_parent) in &mut text_query {
                    if text_parent.get() == bomb_entity {
                        text.sections[0].value = String::from("!");
                        if let Some(bomb_colors) = bomb.to_colors() {
                            text.sections[0].style.color = bomb_colors.text;
                            light.color = bomb_colors.text;
                            bomb_sprite.color = bomb_colors.bomb;
                        }
                    }
                }
            }
        }
    }
}

fn manage_bomb_explosion_side_effects(
    mut explosions_listener: EventReader<BombExploded>,
    mut sounds_event_writer: EventWriter<SoundEvent>,
    mut update_player_score_event_writer: EventWriter<AppendToPlayerScoreEvent>,
    mut game_event_writer: EventWriter<GameEvent>,
    sprites_atlas_resource: Res<StaticImageHandles>,
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
                sprite: Sprite {
                    color: Color::from(DARK_GRAY),
                    ..default()
                },
                ..default()
            },
            InWorldButNotBoundWrapped,
        ));

        if exploded_bomb.monster_hit_count > 0 {
            update_player_score_event_writer.send(AppendToPlayerScoreEvent(
                PLAYER_SCORE_POINTS_ON_MONSTER_KILLED * exploded_bomb.monster_hit_count as u32,
            ));
        }
        if exploded_bomb.hit_player {
            game_event_writer.send(GameEvent::GameOver);
        }
    }
}
