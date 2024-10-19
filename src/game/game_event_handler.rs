use crate::{prelude::*, read_no_field_variant};

pub struct GameEventHandlerPlugin;

impl Plugin for GameEventHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                listen_for_done_timers_with_game_events
                    .in_set(GameRestartSystemSet::ScreenDoneFadingListening),
                despawn_all_upon_restart.in_set(GameRestartSystemSet::Despawning),
            ),
        );
    }
}

fn listen_for_done_timers_with_game_events(
    mut timer_done_listener: EventReader<TimerDoneEvent>,
    mut game_event_writer: EventWriter<GameEvent>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
) {
    for done_timer in timer_done_listener.read() {
        if let TimerDoneEventType::GameEvent(game_event) = done_timer.event_type {
            game_event_writer.send(game_event);
            time_multiplier_request_writer.send(SetTimeMultiplier {
                multiplier_id: TimeMultiplierId::GameTimeMultiplier,
                new_multiplier: 1.0,
                duration: AGAIN_SCREEN_FADE_TIME,
            });
        }
    }
}

fn despawn_all_upon_restart(
    mut event_reader: EventReader<GameEvent>,
    border_crossers_query: Query<Entity, With<WorldBoundsWrapped>>,
    border_non_crossers_query: Query<Entity, With<InWorldButNotBoundWrapped>>,
    timer_query: Query<(Entity, &EmittingTimer)>,
    timer_sequence_query: Query<Entity, With<TimerSequence>>,
    mut commands: Commands,
) {
    if read_no_field_variant!(event_reader, GameEvent::RestartGame).count() > 0 {
        for border_crosser in &border_crossers_query {
            despawn_recursive_notify_on_fail(
                border_crosser,
                "border crosser when pending restart",
                &mut commands,
            );
        }
        for border_non_crosser in &border_non_crossers_query {
            despawn_recursive_notify_on_fail(
                border_non_crosser,
                "border non-crosser when pending restart",
                &mut commands,
            );
        }
        for (timer_entity, timer) in &timer_query {
            for calculator in timer.calculator_entities_iter() {
                despawn_recursive_notify_on_fail(
                    calculator,
                    "calculator when pending restart",
                    &mut commands,
                );
            }
            despawn_recursive_notify_on_fail(
                timer_entity,
                "timer when pending restart",
                &mut commands,
            );
        }
        for timer_sequence in &timer_sequence_query {
            despawn_recursive_notify_on_fail(
                timer_sequence,
                "timer sequence when pending restart",
                &mut commands,
            );
        }
    }
}
