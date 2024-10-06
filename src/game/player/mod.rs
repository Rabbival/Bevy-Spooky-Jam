use crate::prelude::*;

pub mod player_event_channel;
pub mod player_movement;
pub mod player_spawner;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerSpawnerPlugin,
            PlayerMovemetPlugin,
            PlayerRequestPlugin,
        ));
    }
}
