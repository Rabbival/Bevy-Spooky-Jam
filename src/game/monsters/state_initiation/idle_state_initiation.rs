use crate::prelude::*;

pub struct MonsterIdleStateInitiationPlugin;

impl Plugin for MonsterIdleStateInitiationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_change_to_idle_requests.in_set(MonsterSystemSet::StateChanging),
        );
    }
}

fn listen_for_change_to_idle_requests(
    mut monster_state_set_listener: EventReader<MonsterStateSetRequest>,
    mut timer_done_event_writer: EventWriter<TimerDoneEvent>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut monsters_query: Query<(
        &mut Monster,
        &Transform,
        &Handle<ColorMaterial>,
        Entity,
        &AffectingTimerCalculators,
    )>,
    emitting_timer_with_parent_sequence_query: Query<(&EmittingTimer, &TimerParentSequence)>,
    assets: Res<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for request in monster_state_set_listener.read() {
        if let MonsterState::Idle = request.next_state {
            match monsters_query.get_mut(request.monster) {
                Ok((
                    mut monster,
                    monster_transform,
                    monster_color_handle,
                    monster_entity,
                    affecting_timer_calculators,
                )) => {
                    monster.state = request.next_state;
                    if cancel_stray_path_timer_and_begin_next_path_one(
                        &mut timer_done_event_writer,
                        &monster,
                        affecting_timer_calculators,
                        &emitting_timer_with_parent_sequence_query,
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
                        visualize_init_initiation(
                            &mut timer_fire_request_writer,
                            monster_entity,
                            monster_transform.scale,
                            monster_color.color.alpha(),
                            &mut commands,
                        );
                    }
                }
                Err(_) => {
                    print_error(
                        EntityError::EntityNotInQuery("monster when asked to initiate idle state"),
                        vec![LogCategory::RequestNotFulfilled, LogCategory::Monster],
                    );
                }
            }
        }
    }
}

fn cancel_stray_path_timer_and_begin_next_path_one(
    timer_done_event_writer: &mut EventWriter<TimerDoneEvent>,
    monster: &Monster,
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timer_with_parent_sequence_query: &Query<(&EmittingTimer, &TimerParentSequence)>,
    commands: &mut Commands,
) -> Option<TimerDoneEvent> {
    let maybe_timer_done_event = despawn_stray_path_timer_and_get_done_event(
        monster,
        affecting_timer_calculators,
        emitting_timer_with_parent_sequence_query,
        commands,
    );
    if let Some(timer_done_event) = maybe_timer_done_event {
        timer_done_event_writer.send(timer_done_event);
    }
    maybe_timer_done_event
}

fn despawn_stray_path_timer_and_get_done_event(
    monster: &Monster,
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timer_with_parent_sequence_query: &Query<(&EmittingTimer, &TimerParentSequence)>,
    commands: &mut Commands,
) -> Option<TimerDoneEvent> {
    if let Some(direct_line_movers) =
        affecting_timer_calculators.get(&TimerGoingEventType::Move(MovementType::InDirectLine))
    {
        for timer_and_calculator in direct_line_movers {
            let timer_entity = timer_and_calculator.timer;
            if let Ok((timer, parent_sequence)) =
                emitting_timer_with_parent_sequence_query.get(timer_and_calculator.timer)
            {
                if monster.path_timer_sequence == parent_sequence.parent_sequence {
                    despawn_recursive_notify_on_fail(
                        timer_and_calculator.timer,
                        "timer when changing monster state",
                        commands,
                    );
                    return Some(TimerDoneEvent {
                        event_type: timer.send_once_done,
                        affected_entities: timer.affected_entities,
                        timer_entity,
                        timer_parent_sequence: Some(*parent_sequence),
                    });
                }
            }
        }
    }
    None
}

fn visualize_init_initiation(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    monster_entity: Entity,
    monster_scale: Vec3,
    monster_alpha: f32,
    commands: &mut Commands,
) {
    let alpha_calculator = spawn_monster_calm_down_alpha_set_calculator(monster_alpha, commands);
    let scale_calculator = spawn_monster_calm_down_scale_change_calculator(monster_scale, commands);
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

fn spawn_monster_calm_down_alpha_set_calculator(
    monster_current_alpha: f32,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::KeepNewTimer,
            ValueByInterpolation::from_goal_and_current(
                monster_current_alpha,
                MONSTER_FADED_ALPHA,
                Interpolator::new(2.0),
            ),
            TimerGoingEventType::SetAlpha,
        ))
        .id()
}

fn spawn_monster_calm_down_scale_change_calculator(
    monster_current_size: Vec3,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::KeepNewTimer,
            ValueByInterpolation::from_goal_and_current(
                monster_current_size,
                Vec3::ONE,
                Interpolator::new(2.0),
            ),
            TimerGoingEventType::Scale,
        ))
        .id()
}
