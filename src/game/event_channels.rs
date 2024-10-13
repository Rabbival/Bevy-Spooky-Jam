use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub enum GameEvent {
    RestartGame,
    PauseGame,
}

pub struct GameEventPlugin;

impl Plugin for GameEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<GameEvent>();
    }
}
