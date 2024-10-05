use crate::prelude::*;

pub mod consts;
pub mod event_channels;
pub mod orb;
pub mod patroller;
pub mod player;
pub mod tags;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            OrbPlugin,
            PatrollerPlugin,
            GameEventChannelsPlugin,
        ));
    }
}
