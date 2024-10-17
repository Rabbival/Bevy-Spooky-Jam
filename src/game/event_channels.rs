use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub enum GameEvent {
    RestartGame,
    DebugKeyPressed,
}

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct GameOver;

pub struct GameEventPlugin;

impl Plugin for GameEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvent>().add_event::<GameOver>();
    }
}
