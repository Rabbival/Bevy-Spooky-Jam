use crate::{prelude::*, read_no_field_variant};

pub struct BombSpawningSequenceManagerPlugin;

impl Plugin for BombSpawningSequenceManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_bomb_spawner_timer_sequence)
            .add_systems(
                Update,
                respawn_bombs_spawner_on_game_restart.in_set(GameRestartSystemSet::Respawning),
            );
    }
}

fn respawn_bombs_spawner_on_game_restart(
    mut event_reader: EventReader<GameEvent>,
    timer_fire_event_writer: EventWriter<TimerFireRequest>,
    commands: Commands,
) {
    for _restart_event in read_no_field_variant!(event_reader, GameEvent::RestartGame) {
        spawn_bomb_spawner_timer_sequence(timer_fire_event_writer, commands);
        break;
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
