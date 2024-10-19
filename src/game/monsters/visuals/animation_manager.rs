use crate::prelude::*;

pub struct MonsterAnimationManagerPlugin;

impl Plugin for MonsterAnimationManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                (
                    listen_for_spawn_phase_ending,
                    listen_for_done_main_path_timers,
                )
                    .in_set(MonsterSystemSet::PathAndVisualUpdating),
                listen_for_stray_path_changes.in_set(MonsterSystemSet::PostPathUpdating),
            ),
        );
    }
}

fn listen_for_spawn_phase_ending(
    mut monster_state_change_event: EventReader<MonsterStateChanged>,
    mut timer_fire_writer: EventWriter<TimerFireRequest>,
    mut monsters_query: Query<&mut Monster>,
    mut commands: Commands,
) {
    for event in monster_state_change_event.read() {
        if let MonsterState::Spawning = event.previous_state {
            if event.next_state == MonsterState::default() {
                if let Ok(mut monster) = monsters_query.get_mut(event.monster) {
                    match spawn_and_fire_animation_timer_sequence(
                        monster.heading_direction_by_index(0),
                        &mut timer_fire_writer,
                        event.monster,
                        &mut commands,
                    ) {
                        Ok(timer_sequence_entity) => {
                            monster.animation_timer_sequence = Some(timer_sequence_entity);
                        }
                        Err(timer_sequence_error) => {
                            print_error(
                                timer_sequence_error,
                                vec![LogCategory::RequestNotFulfilled, LogCategory::Time],
                            );
                        }
                    }
                }
            }
        }
    }
}

fn listen_for_done_main_path_timers(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    timer_sequence_query: Query<&TimerSequence>,
    vec3_timer_calculators: Query<&GoingEventValueCalculator<Vec3>>,
    mut timer_fire_writer: EventWriter<TimerFireRequest>,
    mut monsters_query: Query<&mut Monster>,
    mut commands: Commands,
) {
    for event in timer_done_event_reader.read() {
        if let TimerDoneEventType::SetAnimationCycleByPathParentSequence = event.event_type {
            if let Some(timer_parent_sequence) = event.timer_parent_sequence {
                for monster_entity in event.affected_entities.affected_entities_iter() {
                    match fire_next_path_timer_animation_sequence(
                        monster_entity,
                        &timer_sequence_query,
                        timer_parent_sequence,
                        &vec3_timer_calculators,
                        &mut timer_fire_writer,
                        &mut commands,
                    ) {
                        Err(timer_sequence_error) => {
                            print_warning(
                                timer_sequence_error,
                                vec![LogCategory::RequestNotFulfilled],
                            );
                        }
                        Ok(new_animation_sequence_entity) => {
                            if let Ok(mut monster) = monsters_query.get_mut(monster_entity) {
                                replace_timer_sequence_for_monster(
                                    new_animation_sequence_entity,
                                    &mut monster,
                                    &mut commands,
                                );
                            }
                        }
                    }
                }
            } else {
                print_warning("Timer with event SetAnimationCycleByPathParentSequence finished but it has no parent sequence", vec![LogCategory::RequestNotFulfilled]);
            }
        }
    }
}

fn fire_next_path_timer_animation_sequence(
    monster_entity: Entity,
    timer_sequence_query: &Query<&TimerSequence>,
    timer_parent_sequence: TimerParentSequence,
    vec3_timer_calculators: &Query<&GoingEventValueCalculator<Vec3>>,
    timer_fire_writer: &mut EventWriter<TimerFireRequest>,
    commands: &mut Commands,
) -> Result<Entity, TimerSequenceError> {
    if let Ok(path_sequence) = timer_sequence_query.get(timer_parent_sequence.parent_sequence) {
        let sequence_status =
            path_sequence.get_next_timer_index(timer_parent_sequence.index_in_sequence);
        if let Some(next_timer_index) = sequence_status.next_timer_index {
            let next_timer = path_sequence.get_timer_by_index(next_timer_index)?;
            let maybe_next_direction =
                get_direction_facing_from_timer(next_timer, vec3_timer_calculators);
            if let Some(next_facing_direction) = maybe_next_direction {
                return spawn_and_fire_animation_timer_sequence(
                    next_facing_direction,
                    timer_fire_writer,
                    monster_entity,
                    commands,
                );
            }
        }
    }
    Err(TimerSequenceError::ATimerFromASequenceFinishedButParentNotFound)
}

fn get_direction_facing_from_timer(
    timer: EmittingTimer,
    vec3_timer_calculators: &Query<&GoingEventValueCalculator<Vec3>>,
) -> Option<BasicDirection> {
    for calculator_entity in timer.calculator_entities_iter() {
        if let Ok(calculator) = vec3_timer_calculators.get(calculator_entity) {
            if let TimerGoingEventType::Move(_) = calculator.going_event_type() {
                return Some(BasicDirection::closest(
                    calculator.get_full_delta().truncate(),
                ));
            }
        }
    }
    None
}

fn listen_for_stray_path_changes(
    mut stray_path_change_listener: EventReader<MonsterStrayPathUpdated>,
    mut timer_fire_writer: EventWriter<TimerFireRequest>,
    mut monsters_query: Query<&mut Monster>,
    mut commands: Commands,
) {
    for event in stray_path_change_listener.read() {
        if let Ok(mut monster) = monsters_query.get_mut(event.monster_entity) {
            match spawn_and_fire_animation_timer_sequence(
                BasicDirection::closest(event.new_delta),
                &mut timer_fire_writer,
                event.monster_entity,
                &mut commands,
            ) {
                Err(timer_sequence_error) => {
                    print_warning(timer_sequence_error, vec![LogCategory::RequestNotFulfilled]);
                }
                Ok(new_animation_sequence) => {
                    replace_timer_sequence_for_monster(
                        new_animation_sequence,
                        &mut monster,
                        &mut commands,
                    );
                }
            }
        }
    }
}

fn spawn_and_fire_animation_timer_sequence(
    heading_direction: BasicDirection,
    timer_fire_writer: &mut EventWriter<TimerFireRequest>,
    monster_entity: Entity,
    commands: &mut Commands,
) -> Result<Entity, TimerSequenceError> {
    let initial_fram_index = heading_direction.to_monster_initial_frame_index();
    let mut frame_vec = vec![];
    for index_offset in [0, 9, 18, 9] {
        frame_vec.push(initial_fram_index + index_offset);
    }
    let timer_sequence = FrameSequence::looping_frame_sequence(
        vec![monster_entity],
        vec![TimeMultiplierId::GameTimeMultiplier],
        FLYING_FRAME_LOOP_DURATION,
        frame_vec,
    )
    .0;
    let sequence_entity = commands.spawn(timer_sequence).id();
    timer_sequence.fire_first_timer(sequence_entity, timer_fire_writer)?;
    Ok(sequence_entity)
}

fn replace_timer_sequence_for_monster(
    new_animation_sequence_entity: Entity,
    monster: &mut Monster,
    commands: &mut Commands,
) {
    if let Some(current_animation_sequence) = monster.animation_timer_sequence {
        despawn_recursive_notify_on_fail(
            current_animation_sequence,
            "monster animation sequence",
            commands,
        );
    }
    monster.animation_timer_sequence = Some(new_animation_sequence_entity);
}
