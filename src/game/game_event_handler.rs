use crate::{prelude::*, read_no_field_variant};

pub struct GameEventHandlerPlugin;

impl Plugin for GameEventHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            despawn_all_upon_restart.in_set(GameRestartSystemSet::Despawning),
        );
    }
}

fn despawn_all_upon_restart(
    mut event_reader: EventReader<GameEvent>,
    border_crossers_query: Query<Entity, With<WorldBoundsWrapped>>,
    timer_query: Query<(Entity, &EmittingTimer)>,
    timer_sequence_query: Query<Entity, With<TimerSequence>>,
    mut commands: Commands,
) {
    for _restart_event in read_no_field_variant!(event_reader, GameEvent::RestartGame) {
        for border_crosser in &border_crossers_query {
            despawn_recursive_notify_on_fail(
                border_crosser,
                "border crosser when pending restart",
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
        break;
    }
}
