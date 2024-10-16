use crate::prelude::*;

pub struct MonsterAnimationStarterPlugin;

impl Plugin for MonsterAnimationStarterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_spawn_phase_ending.in_set(MonsterSystemSet::PathAndVisualUpdating),
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
