use crate::prelude::*;

pub struct MonsterChaseStateInitiationPlugin;

impl Plugin for MonsterChaseStateInitiationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_change_to_chasing_requests.in_set(MonsterSystemSet::StateChanging),
        );
    }
}

fn listen_for_change_to_chasing_requests(
    mut monster_state_set_listener: EventReader<MonsterStateSetRequest>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut monsters_query: Query<(
        &mut Monster,
        &Transform,
        &Handle<ColorMaterial>,
        Entity,
        &AffectingTimerCalculators,
    )>,
    transform_query: Query<&Transform>,
    emitting_timer_parent_sequence_query: Query<&TimerParentSequence, With<EmittingTimer>>,
    assets: Res<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for request in monster_state_set_listener.read() {
        if let MonsterState::Chasing(entity_chasing) = request.next_state {
            match monsters_query.get_mut(request.monster) {
                Ok((
                    mut monster,
                    monster_transform,
                    monster_color_handle,
                    monster_entity,
                    affecting_timer_calculators,
                )) => {
                    monster.state = request.next_state;
                    if let Ok(transform_chasing) = transform_query.get(entity_chasing) {
                        if replace_path_with_chasing_path(
                            &mut timer_fire_request_writer,
                            &monster,
                            monster_entity,
                            monster_transform.translation,
                            transform_chasing.translation,
                            affecting_timer_calculators,
                            &emitting_timer_parent_sequence_query,
                            &mut commands,
                        )
                        .is_none()
                        {
                            print_error(
                                MonsterError::NoPathSequenceFoundOnStateChange(request.next_state),
                                vec![LogCategory::RequestNotFulfilled, LogCategory::Monster],
                            );
                        }
                        if let Some(monster_color) = assets.get(monster_color_handle.id()) {
                            visualize_chase_initiation(
                                &mut timer_fire_request_writer,
                                monster_entity,
                                monster_transform.scale,
                                monster_color.color.alpha(),
                                &mut commands,
                            );
                        }
                    }
                }
                Err(_) => {
                    print_error(
                        EntityError::EntityNotInQuery("monster when asked to intiate chase"),
                        vec![LogCategory::RequestNotFulfilled, LogCategory::Monster],
                    );
                }
            }
        }
    }
}

fn replace_path_with_chasing_path(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    monster: &Monster,
    monster_entity: Entity,
    monster_location: Vec3,
    location_to_chase: Vec3,
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timer_parent_sequence_query: &Query<&TimerParentSequence, With<EmittingTimer>>,
    commands: &mut Commands,
) -> Option<TimerParentSequence> {
    let maybe_path_timer_parent_sequence = destroy_current_path_timer_and_calculator(
        monster,
        affecting_timer_calculators,
        emitting_timer_parent_sequence_query,
        commands,
    );
    if let Some(path_timer_parent_sequence) = maybe_path_timer_parent_sequence {
        let distance_to_goal = monster_location.distance(location_to_chase);
        let move_calculator = spawn_monster_chase_start_move_calculator(
            monster_location,
            location_to_chase,
            commands,
        );
        timer_fire_request_writer.send(TimerFireRequest {
            timer: EmittingTimer::new(
                vec![TimerAffectedEntity {
                    affected_entity: monster_entity,
                    value_calculator_entity: Some(move_calculator),
                }],
                vec![TimeMultiplierId::GameTimeMultiplier],
                distance_to_goal / MONSTER_SPEED_WHEN_CHASING,
                TimerDoneEventType::Nothing,
            ),
            parent_sequence: Some(path_timer_parent_sequence),
        });
    }
    maybe_path_timer_parent_sequence
}

fn destroy_current_path_timer_and_calculator(
    monster: &Monster,
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timer_parent_sequence_query: &Query<&TimerParentSequence, With<EmittingTimer>>,
    commands: &mut Commands,
) -> Option<TimerParentSequence> {
    if let Some(direct_line_movers) =
        affecting_timer_calculators.get(&TimerGoingEventType::Move(MovementType::InDirectLine))
    {
        for timer_and_calculator in direct_line_movers {
            if let Ok(parent_sequence) =
                emitting_timer_parent_sequence_query.get(timer_and_calculator.timer)
            {
                if monster.path_timer_sequence == parent_sequence.parent_sequence {
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

fn spawn_monster_chase_start_move_calculator(
    current_location: Vec3,
    location_to_chase: Vec3,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::AppendToTimersOfType,
            ValueByInterpolation::from_goal_and_current(
                current_location,
                location_to_chase,
                Interpolator::new(1.0),
            ),
            TimerGoingEventType::Move(MovementType::InDirectLine),
        ))
        .id()
}

fn visualize_chase_initiation(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    monster_entity: Entity,
    monster_scale: Vec3,
    monster_alpha: f32,
    commands: &mut Commands,
) {
    let alpha_calculator = spawn_monster_chase_start_alpha_set_calculator(monster_alpha, commands);
    let scale_calculator =
        spawn_monster_chase_start_scale_change_calculator(monster_scale, commands);
    timer_fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![
                TimerAffectedEntity {
                    affected_entity: monster_entity,
                    value_calculator_entity: Some(alpha_calculator),
                },
                TimerAffectedEntity {
                    affected_entity: monster_entity,
                    value_calculator_entity: Some(scale_calculator),
                },
            ],
            vec![TimeMultiplierId::GameTimeMultiplier],
            MONSTER_CHASE_START_VISUAL_CHANGE_DURATION,
            TimerDoneEventType::Nothing,
        ),
        parent_sequence: None,
    });
}

fn spawn_monster_chase_start_alpha_set_calculator(
    monster_current_alpha: f32,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::KeepNewTimer,
            ValueByInterpolation::from_goal_and_current(
                monster_current_alpha,
                1.0,
                Interpolator::new(0.4),
            ),
            TimerGoingEventType::SetAlpha,
        ))
        .id()
}

fn spawn_monster_chase_start_scale_change_calculator(
    monster_current_size: Vec3,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::KeepNewTimer,
            ValueByInterpolation::from_goal_and_current(
                monster_current_size,
                MONSTER_SIZE_WHEN_CHASING * Vec3::ONE,
                Interpolator::new(0.2),
            ),
            TimerGoingEventType::Scale,
        ))
        .id()
}
