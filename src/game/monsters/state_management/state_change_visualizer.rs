use crate::prelude::*;

pub struct MonsterStateChangeVisualizerPlugin;

impl Plugin for MonsterStateChangeVisualizerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_to_state_changes.in_set(MonsterSystemSet::PathUpdating),
        );
    }
}

fn listen_to_state_changes(
    mut monster_state_set_listener: EventReader<MonsterStateChanged>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    monsters_query: Query<(&Transform, &Handle<ColorMaterial>, Entity)>,
    assets: Res<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for request in monster_state_set_listener.read() {
        match monsters_query.get(request.monster) {
            Ok((monster_transform, monster_color_handle, monster_entity)) => {
                if let Some(monster_color) = assets.get(monster_color_handle.id()) {
                    determine_visualize_change_and_initiate_if_required(
                        &mut timer_fire_request_writer,
                        monster_entity,
                        monster_transform.scale,
                        monster_color.color.alpha(),
                        request,
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

fn determine_visualize_change_and_initiate_if_required(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    monster_entity: Entity,
    monster_scale: Vec3,
    monster_alpha: f32,
    state_change_event: &MonsterStateChanged,
    commands: &mut Commands,
) {
    let mut maybe_scaler = None;
    let mut maybe_alpha_changer = None;
    if should_apply_chase_visuals(state_change_event) {
        maybe_scaler = Some(spawn_chase_initiation_scale_calculator(
            monster_scale,
            commands,
        ));
        maybe_alpha_changer = Some(spawn_chase_initiation_alpha_calculator(
            monster_alpha,
            commands,
        ));
    } else if should_cancel_chase_visuals(state_change_event) {
        maybe_scaler = Some(spawn_chase_cancel_scale_calculator(monster_scale, commands));
        maybe_alpha_changer = Some(spawn_chase_cancel_alpha_calculator(monster_alpha, commands));
    }
    if let Some(scale_calculator) = maybe_scaler {
        if let Some(alpha_calculator) = maybe_alpha_changer {
            fire_state_change_visualizer(
                timer_fire_request_writer,
                monster_entity,
                alpha_calculator,
                scale_calculator,
            );
        }
    }
}

fn fire_state_change_visualizer(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    monster_entity: Entity,
    alpha_calculator: Entity,
    scale_calculator: Entity,
) {
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

fn spawn_chase_initiation_alpha_calculator(
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

fn spawn_chase_initiation_scale_calculator(
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

fn spawn_chase_cancel_alpha_calculator(
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

fn spawn_chase_cancel_scale_calculator(
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

fn should_apply_chase_visuals(state_changed_event: &MonsterStateChanged) -> bool {
    if let MonsterState::Fleeing(_) | MonsterState::Idle = state_changed_event.previous_state {
        if let MonsterState::Chasing(_) = state_changed_event.next_state {
            return true;
        }
    }
    false
}

fn should_cancel_chase_visuals(state_changed_event: &MonsterStateChanged) -> bool {
    if let MonsterState::Chasing(_) = state_changed_event.previous_state {
        if let MonsterState::Fleeing(_) | MonsterState::Idle = state_changed_event.next_state {
            return true;
        }
    }
    false
}