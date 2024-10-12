use crate::prelude::*;

pub mod consts;
pub mod player_event_channel;
pub mod player_movement;
pub mod player_spawner;
pub mod tags;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerSpawnerPlugin,
            PlayerMovementPlugin,
            PlayerRequestPlugin,
        ));
    }
}
