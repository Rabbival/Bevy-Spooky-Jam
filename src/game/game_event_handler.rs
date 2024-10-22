use crate::{prelude::*, read_no_field_variant};

pub struct GameEventHandlerPlugin;

impl Plugin for GameEventHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, tag_all_prestartup_entities_with_destorynt)
            .add_systems(
                Update,
                (
                    listen_for_done_timers_with_game_events
                        .in_set(GameRestartSystemSet::ScreenDoneFadingListening),
                    despawn_all_upon_restart.in_set(GameRestartSystemSet::Despawning),
                ),
            );
    }
}

fn tag_all_prestartup_entities_with_destorynt(all_entities: Query<Entity>, mut commands: Commands) {
    for entity in &all_entities {
        commands.entity(entity).insert(DoNotDestroyOnRestart);
    }
}

fn listen_for_done_timers_with_game_events(
    mut timer_done_listener: EventReader<TimerDoneEvent>,
    mut game_event_writer: EventWriter<GameEvent>,
) {
    for done_timer in timer_done_listener.read() {
        if let TimerDoneEventType::GameEvent(game_event) = done_timer.event_type {
            game_event_writer.send(game_event);
        }
    }
}

fn despawn_all_upon_restart(
    mut event_reader: EventReader<GameEvent>,
    entities_to_destroy: Query<Entity, Without<DoNotDestroyOnRestart>>,
    mut commands: Commands,
) {
    if read_no_field_variant!(event_reader, GameEvent::RestartGame).count() > 0 {
        for doomed_entity in &entities_to_destroy {
            despawn_recursive_notify_on_fail(
                doomed_entity,
                "entity when pending restart",
                &mut commands,
            );
        }
    }
}
