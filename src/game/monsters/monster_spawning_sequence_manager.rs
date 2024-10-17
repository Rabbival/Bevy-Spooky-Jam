use crate::{prelude::*, read_no_field_variant};

#[derive(Resource, Debug, Default)]
pub struct MonsterSpawnSequence(pub Option<Entity>);

pub struct MonsterSpawningSequenceManagerPlugin;

impl Plugin for MonsterSpawningSequenceManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MonsterSpawnSequence>()
            .add_systems(Startup, spawn_monster_spawner_timer_sequence)
            .add_systems(
                Update,
                (
                    respawn_monsters_spawner_on_game_restart.in_set(GameRestartSystemSet::Spawning),
                    accelerate_upon_monster_spawn,
                ),
            );
    }
}

fn respawn_monsters_spawner_on_game_restart(
    mut event_reader: EventReader<GameEvent>,
    timer_fire_event_writer: EventWriter<TimerFireRequest>,
    monster_sequence_resource: ResMut<MonsterSpawnSequence>,
    commands: Commands,
) {
    for _restart_event in read_no_field_variant!(event_reader, GameEvent::RestartGame) {
        spawn_monster_spawner_timer_sequence(
            timer_fire_event_writer,
            monster_sequence_resource,
            commands,
        );
        break;
    }
}

fn spawn_monster_spawner_timer_sequence(
    mut timer_fire_event_writer: EventWriter<TimerFireRequest>,
    mut monster_sequence_resource: ResMut<MonsterSpawnSequence>,
    mut commands: Commands,
) {
    let maybe_newborn_sequence = TimerSequence::spawn_looping_sequence_and_fire_first_timer(
        &mut timer_fire_event_writer,
        &vec![EmittingTimer::new(
            vec![],
            vec![TimeMultiplierId::GameTimeMultiplier],
            MONSTER_SPAWN_INITIAL_INTERVAL,
            TimerDoneEventType::Spawn(SpawnRequestType::Monster),
        )],
        &mut commands,
    );
    match maybe_newborn_sequence {
        Err(sequence_error) => {
            print_error(
                sequence_error,
                vec![LogCategory::RequestNotFulfilled, LogCategory::Time],
            );
        }
        Ok(sequence_entity) => monster_sequence_resource.0 = Some(sequence_entity),
    }
}

fn accelerate_upon_monster_spawn(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    mut sequences_query: Query<&mut TimerSequence>,
    monster_sequence_resource: Res<MonsterSpawnSequence>,
) {
    for event in timer_done_event_reader.read() {
        if let TimerDoneEventType::Spawn(SpawnRequestType::Monster) = event.event_type {
            if let Some(sequence_entity) = monster_sequence_resource.0 {
                if let Ok(mut sequence) = sequences_query.get_mut(sequence_entity) {
                    if let Ok(timer) = sequence.get_timer_by_index(0) {
                        let mut shorter_timer = timer;
                        shorter_timer.duration = max(
                            timer.duration - MONSTER_SPAWN_INTERVAL_DELTA,
                            MONSTER_SPAWN_MINIMAL_INTERVAL,
                        );
                        sequence.timers_in_order.array[0] = Some(shorter_timer);
                    }
                }
            }
        }
    }
}
