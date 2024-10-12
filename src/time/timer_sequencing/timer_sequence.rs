use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct TimerSequence {
    pub timers_in_order: VecBasedArray<EmittingTimer, MAX_TIMERS_IN_SEQUENCE>,
    pub loop_back_to_start: bool,
}

impl TimerSequence {
    pub fn spawn_looping_sequence_and_fire_first_timer(
        timer_fire_event_writer: &mut EventWriter<TimerFireRequest>,
        timers_in_order: &[EmittingTimer],
        commands: &mut Commands,
    ) -> Result<Entity, TimerSequenceError> {
        TimerSequence::spawn_sequence_and_fire_first_timer(
            timer_fire_event_writer,
            timers_in_order,
            true,
            commands,
        )
    }

    pub fn spawn_non_looping_sequence_and_fire_first_timer(
        timer_fire_event_writer: &mut EventWriter<TimerFireRequest>,
        timers_in_order: &[EmittingTimer],
        commands: &mut Commands,
    ) -> Result<Entity, TimerSequenceError> {
        TimerSequence::spawn_sequence_and_fire_first_timer(
            timer_fire_event_writer,
            timers_in_order,
            false,
            commands,
        )
    }

    fn spawn_sequence_and_fire_first_timer(
        timer_fire_event_writer: &mut EventWriter<TimerFireRequest>,
        timers_in_order: &[EmittingTimer],
        loop_back_to_start: bool,
        commands: &mut Commands,
    ) -> Result<Entity, TimerSequenceError> {
        if timers_in_order.first().is_some() {
            let newborn_sequence = Self::new(timers_in_order, loop_back_to_start);
            let newborn_sequence_id = commands.spawn(newborn_sequence).id();
            newborn_sequence.fire_first_timer(newborn_sequence_id, timer_fire_event_writer)?;
            Ok(newborn_sequence_id)
        } else {
            Err(TimerSequenceError::TriedToFireATimerSequenceWithNoTimers)
        }
    }

    pub fn fire_first_timer(
        &self,
        sequence: Entity,
        timer_fire_event_writer: &mut EventWriter<TimerFireRequest>,
    ) -> Result<(), TimerSequenceError> {
        if self.timers_in_order.is_empty() {
            Err(TimerSequenceError::TriedToFireATimerSequenceWithNoTimers)
        } else {
            timer_fire_event_writer.send(TimerFireRequest {
                timer: self.timers_in_order.array[0].unwrap(),
                parent_sequence: Some(TimerParentSequence {
                    parent_sequence: sequence,
                    index_in_sequence: 0,
                }),
            });
            Ok(())
        }
    }

    pub fn looping_sequence(timers_in_order: &[EmittingTimer]) -> TimerSequence {
        TimerSequence::new(timers_in_order, true)
    }

    pub fn non_looping_sequence(timers_in_order: &[EmittingTimer]) -> TimerSequence {
        TimerSequence::new(timers_in_order, false)
    }

    fn new(timers_in_order: &[EmittingTimer], loop_back_to_start: bool) -> TimerSequence {
        let timers_in_order_array = VecBasedArray::new(timers_in_order.to_vec());
        TimerSequence {
            timers_in_order: timers_in_order_array,
            loop_back_to_start,
        }
    }

    pub fn get_timer_by_index(&self, index: usize) -> Result<EmittingTimer, TimerSequenceError> {
        match self.timers_in_order.array[index] {
            Some(timer) => Ok(timer),
            None => Err(TimerSequenceError::SequenceHasNoTimerInIndex(index)),
        }
    }

    pub fn get_next_timer_index(&self, done_timer_index: usize) -> TimerSequenceStatus {
        let next_index = done_timer_index + 1;
        let sequence_timer_count = self.timers_in_order.len();
        if next_index >= sequence_timer_count {
            if self.loop_back_to_start {
                TimerSequenceStatus {
                    next_timer_index: Some(0),
                    sequence_done: false,
                }
            } else {
                TimerSequenceStatus {
                    next_timer_index: None,
                    sequence_done: true,
                }
            }
        } else {
            TimerSequenceStatus {
                next_timer_index: Some(next_index),
                sequence_done: false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::query::QuerySingleError;

    #[derive(Resource)]
    struct TimerSequenceToSpawn {
        timers_in_order: Vec<EmittingTimer>,
        loop_back_to_start: bool,
    }

    #[test]
    #[should_panic(expected = "spawn attempt return an error")]
    fn nothing_spawns_if_timer_list_is_empty() {
        let mut app = App::new();
        attempt_timer_sequence_spawning(&mut app, &[], false);
    }

    #[test]
    fn one_timer_and_sequence_spawn() {
        let mut app = App::new();
        attempt_timer_sequence_spawning(&mut app, &two_emitting_timers_vec(), false);
        assert_eq!(count_timer_firing_requests(&mut app), 1);
        assert_eq!(count_timer_sequences_in_world(&mut app), 1);
    }

    #[test]
    fn test_correct_index_advancing_for_looping_sequence() {
        test_correct_index_advancing(true);
    }

    #[test]
    fn test_correct_index_advancing_for_non_looping_sequence() {
        test_correct_index_advancing(false);
    }

    fn test_correct_index_advancing(looping: bool) {
        let mut app = App::new();
        attempt_timer_sequence_spawning(&mut app, &two_emitting_timers_vec(), looping);
        match try_fetch_single_timer_sequence(&mut app) {
            Ok(timer_sequence) => {
                assert_correct_index_advancing(timer_sequence, looping);
            }
            Err(_) => panic!("failed to fetch single timer sequence"),
        }
    }

    fn assert_correct_index_advancing(sequence: &TimerSequence, looping: bool) {
        let sequence_length = sequence.timers_in_order.len();
        let index_beyond_the_end = if looping { Some(0) } else { None };
        assert_eq!(
            sequence
                .get_next_timer_index(sequence_length - 1)
                .next_timer_index,
            index_beyond_the_end
        );
        assert_eq!(sequence.get_next_timer_index(0).next_timer_index, Some(1));
    }

    fn attempt_timer_sequence_spawning(
        app: &mut App,
        timers_list: &[EmittingTimer],
        loop_back_to_start: bool,
    ) {
        app.insert_resource::<TimerSequenceToSpawn>(TimerSequenceToSpawn {
            timers_in_order: timers_list.to_vec(),
            loop_back_to_start,
        })
        .add_event::<TimerFireRequest>()
        .add_event::<UpdateAffectedEntitiesAfterTimerBirth>()
        .add_systems(
            Update,
            call_spawn_sequence_and_fire_first_timer_with_test_resource,
        );

        app.update();
    }

    fn call_spawn_sequence_and_fire_first_timer_with_test_resource(
        mut timer_fire_event_writer: EventWriter<TimerFireRequest>,
        requested_timer_sequence_properties: Res<TimerSequenceToSpawn>,
        mut commands: Commands,
    ) {
        if let Err(error) = TimerSequence::spawn_sequence_and_fire_first_timer(
            &mut timer_fire_event_writer,
            &requested_timer_sequence_properties.timers_in_order,
            requested_timer_sequence_properties.loop_back_to_start,
            &mut commands,
        ) {
            panic!("spawn attempt return an error: {:?}", error);
        }
    }

    fn two_emitting_timers_vec() -> Vec<EmittingTimer> {
        vec![
            EmittingTimer::new(vec![], vec![], 42.0, TimerDoneEventType::Nothing),
            EmittingTimer::new(vec![], vec![], 42.0, TimerDoneEventType::Nothing),
        ]
    }

    fn count_timer_firing_requests(app: &mut App) -> usize {
        app.world_mut()
            .resource_mut::<Events<TimerFireRequest>>()
            .len()
    }

    fn count_timer_sequences_in_world(app: &mut App) -> usize {
        app.world_mut()
            .query::<&TimerSequence>()
            .iter(app.world())
            .len()
    }

    fn try_fetch_single_timer_sequence(app: &mut App) -> Result<&TimerSequence, QuerySingleError> {
        app.world_mut()
            .query::<&TimerSequence>()
            .get_single(app.world())
    }
}
