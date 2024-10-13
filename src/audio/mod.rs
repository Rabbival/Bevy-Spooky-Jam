use crate::prelude::*;

pub mod music_player;
pub mod sound_event_channel;
pub mod sound_player;
pub mod tags;

pub struct GameAudioPlugin;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MusicPlayerPlugin, SoundPlayerPlugin, SoundEventPlugin))
            .add_systems(Update, game_multiplier_change_listener);
    }
}

fn game_multiplier_change_listener(
    mut multiplier_change_reader: EventReader<TimerGoingEvent<f32>>,
    mut time_affected_audio: Query<(&AffectingTimeMultiplier, &mut AudioSink)>,
    time_multipliers: Query<&TimeMultiplier>,
) {
    for event in multiplier_change_reader.read() {
        if let TimerGoingEventType::ChangeTimeMultiplierSpeed = event.event_type {
            if let Ok(multiplier) = time_multipliers.get(event.entity) {
                for (affecting_time_multiplier, audio) in &mut time_affected_audio {
                    if multiplier.id() == affecting_time_multiplier.0 {
                        let current_speed = audio.speed();
                        audio.set_speed(current_speed + event.value_delta);
                    }
                }
            }
        }
    }
}
