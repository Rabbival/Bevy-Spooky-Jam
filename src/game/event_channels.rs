use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub enum GameEvent {
    RestartGame,
    GameOver,
    DebugKeyPressed,
}

pub struct GameEventPlugin;

impl Plugin for GameEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvent>()
            .add_systems(Update, listen_for_done_timers_with_game_events);
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
