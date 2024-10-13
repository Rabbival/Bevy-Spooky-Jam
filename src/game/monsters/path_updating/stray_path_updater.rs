use crate::prelude::*;

pub struct MonsterStrayPathUpdaterPlugin;

impl Plugin for MonsterStrayPathUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_stray_path, listen_for_state_set).in_set(MonsterSystemSet::PathUpdating),
        );
    }
}

fn listen_for_state_set(
    mut monster_state_set_listener: EventReader<MonsterStateChanged>,
    monsters_query: Query<(Entity, &Monster, &Transform, &AffectingTimerCalculators)>,
    transforms: Query<&Transform>,
    emitting_timer_parent_sequence_query: Query<&TimerParentSequence, With<EmittingTimer>>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut commands: Commands,
) {
    for event in monster_state_set_listener.read() {
        match monsters_query.get(event.monster) {
            Ok((monster_entity, monster, monster_transform, affecting_timer_calculators)) => {
                if let MonsterState::Chasing(target_entity) | MonsterState::Fleeing(target_entity) =
                    event.next_state
                {
                    if let Ok(target_transform) = transforms.get(target_entity) {
                        replace_current_path(
                            target_transform.translation,
                            monster,
                            monster_entity,
                            monster_transform.translation,
                            affecting_timer_calculators,
                            &emitting_timer_parent_sequence_query,
                            &mut timer_fire_request_writer,
                            &mut commands,
                            true,
                        );
                    }
                }
            }
            Err(_) => {
                print_warning(
                    EntityError::EntityNotInQuery(
                        "monster after state change when trying to set path",
                    ),
                    vec![LogCategory::RequestNotFulfilled, LogCategory::Monster],
                );
            }
        }
    }
}

fn update_stray_path(
    just_changed_transforms: Query<&Transform, Changed<Transform>>,
    monsters_query: Query<(Entity, &Monster, &Transform, &AffectingTimerCalculators)>,
    emitting_timer_parent_sequence_query: Query<&TimerParentSequence, With<EmittingTimer>>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut commands: Commands,
) {
    for (monster_entity, monster, monster_transform, affecting_timer_calculators) in &monsters_query
    {
        if let MonsterState::Chasing(target_entity) | MonsterState::Fleeing(target_entity) =
            monster.state
        {
            if let Ok(target_transform) = just_changed_transforms.get(target_entity) {
                replace_current_path(
                    target_transform.translation,
                    monster,
                    monster_entity,
                    monster_transform.translation,
                    affecting_timer_calculators,
                    &emitting_timer_parent_sequence_query,
                    &mut timer_fire_request_writer,
                    &mut commands,
                    false,
                );
            }
        }
    }
}

fn replace_current_path(
    target_location: Vec3,
    monster: &Monster,
    monster_entity: Entity,
    monster_location: Vec3,
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timer_parent_sequence_query: &Query<&TimerParentSequence, With<EmittingTimer>>,
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    commands: &mut Commands,
    destroy_calculator: bool,
) {
    let speed = if let MonsterState::Chasing(_) = monster.state {
        MONSTER_SPEED_WHEN_CHASING
    } else {
        MONSTER_SPEED_WHEN_FLEEING
    };
    let location_to_move_towards = if let MonsterState::Chasing(_) = monster.state {
        target_location
    } else {
        target_location + (monster_location - target_location).normalize() * BOMB_EXPLOSION_RADIUS
    };
    let new_path_calculator =
        spawn_monster_move_calculator(monster_location, location_to_move_towards, commands);
    if let Some(path_timer_parent_sequence) = destroy_current_path_timer_and_calculator(
        monster,
        affecting_timer_calculators,
        emitting_timer_parent_sequence_query,
        commands,
        destroy_calculator,
    ) {
        timer_fire_request_writer.send(TimerFireRequest {
            timer: EmittingTimer::new(
                vec![TimerAffectedEntity {
                    affected_entity: monster_entity,
                    value_calculator_entity: Some(new_path_calculator),
                }],
                vec![TimeMultiplierId::GameTimeMultiplier],
                location_to_move_towards.distance(monster_location) / speed,
                TimerDoneEventType::Nothing,
            ),
            parent_sequence: Some(path_timer_parent_sequence),
        });
    }
}

fn destroy_current_path_timer_and_calculator(
    monster: &Monster,
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timer_parent_sequence_query: &Query<&TimerParentSequence, With<EmittingTimer>>,
    commands: &mut Commands,
    destroy_calculator: bool,
) -> Option<TimerParentSequence> {
    if let Some(direct_line_movers) =
        affecting_timer_calculators.get(&TimerGoingEventType::Move(MovementType::InDirectLine))
    {
        for timer_and_calculator in direct_line_movers {
            if let Ok(parent_sequence) =
                emitting_timer_parent_sequence_query.get(timer_and_calculator.timer)
            {
                if monster.path_timer_sequence == parent_sequence.parent_sequence {
                    if destroy_calculator {
                        despawn_recursive_notify_on_fail(
                            timer_and_calculator.value_calculator,
                            "timer calculator when changing monster state",
                            commands,
                        );
                    }
                    despawn_recursive_notify_on_fail(
                        timer_and_calculator.timer,
                        "timer when changing monster state",
                        commands,
                    );
                    return Some(*parent_sequence);
                }
            }
        }
    }
    None
}

fn spawn_monster_move_calculator(
    current_location: Vec3,
    location_to_move_towards: Vec3,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::AppendToTimersOfType,
            ValueByInterpolation::from_goal_and_current(
                current_location,
                location_to_move_towards.with_z(Z_LAYER_MONSTER),
                Interpolator::new(1.0),
            ),
            TimerGoingEventType::Move(MovementType::InDirectLine),
        ))
        .id()
}
