use crate::prelude::*;

pub mod bombs;
pub mod bounds_wrapped_logic;
pub mod consts;
pub mod event_channels;
pub mod game_event_handler;
pub mod monsters;
pub mod player_management;
pub mod respawner;
pub mod scores;
pub mod tags;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            MonstersPlugin,
            #[cfg(debug_assertions)]
            GizmosPlugin,
            BombsPlugin,
            ScorePlugin,
            GameEventPlugin,
            GameEventHandlerPlugin,
            RespawnerPlugin,
            BoundsWrappedLogicPlugin,
        ));
    }
}
