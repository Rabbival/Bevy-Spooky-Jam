use crate::prelude::*;

pub mod bombs;
pub mod consts;
pub mod event_channels;
pub mod monsters;
pub mod orb;
pub mod player;
pub mod tags;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            MonstersPlugin,
            OrbPlugin,
            BombsPlugin,
            GameEventChannelsPlugin,
        ));
    }
}
