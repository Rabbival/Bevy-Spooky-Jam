use crate::prelude::*;

pub mod consts;
pub mod event_channels;
pub mod orb;
pub mod patroller;
pub mod player;
pub mod tags;
pub mod world_bounds;
pub mod monsters;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            MonstersPlugin,
            OrbPlugin,
            PatrollerPlugin,
            GameEventChannelsPlugin,
            WorldBoundPlugin,
        ));
    }
}
