use crate::audio::sound_event_channel::SoundEventPlugin;
use crate::audio::sound_player::SoundPlayerPlugin;
use crate::prelude::*;

pub mod music_player;
pub mod sound_event_channel;
pub mod sound_player;
pub mod tags;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MusicPlayerPlugin, SoundPlayerPlugin, SoundEventPlugin));
    }
}