use crate::prelude::*;

pub struct BombSpawningSequenceManagerPlugin;

impl Plugin for BombSpawningSequenceManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bomb_spawner_timer_sequence);
    }
}

fn spawn_bomb_spawner_timer_sequence(
    mut timer_fire_event_writer: EventWriter<TimerFireRequest>,
    mut commands: Commands,
) {
    if let Err(sequence_error) = TimerSequence::spawn_looping_sequence_and_fire_first_timer(
        &mut timer_fire_event_writer,
        &vec![EmittingTimer::new(
            vec![],
            vec![TimeMultiplierId::GameTimeMultiplier],
            BOMB_SPAWN_INTERVAL,
            TimerDoneEventType::Spawn(SpawnRequestType::Bomb),
        )],
        &mut commands,
    ) {
        print_error(
            sequence_error,
            vec![LogCategory::RequestNotFulfilled, LogCategory::Time],
        );
    }
}
