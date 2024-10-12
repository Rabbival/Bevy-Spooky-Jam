use crate::prelude::*;

pub mod music_player;
pub mod tags;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MusicPlayerPlugin);
    }
}
