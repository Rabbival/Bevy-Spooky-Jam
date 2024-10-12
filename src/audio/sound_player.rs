use crate::prelude::*;
use bevy::audio::PlaybackMode;

pub struct SoundPlayerPlugin;

impl Plugin for SoundPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bomb_sounds_event_listener);
    }
}

fn bomb_sounds_event_listener(
    sound_assets_resource: Res<SoundAssets>,
    mut sound_events_reader: EventReader<SoundEvent>,
    mut commands: Commands,
) {
    for sound in sound_events_reader.read() {
        let source;
        match sound.event {
            SoundEventEnum::BombExplodeSoundEvent => {
                source = sound_assets_resource.bomb_explode.clone();
            }
            SoundEventEnum::BombPickUpEvent => {
                source = sound_assets_resource.bomb_pick_up.clone();
            }
            SoundEventEnum::BombThrowEvent => {
                source = sound_assets_resource.bomb_throw.clone();
            }
            SoundEventEnum::BombTickEvent => {
                source = sound_assets_resource.bomb_tick.clone();
            }
        }
        if Some(&source).is_some() {
            commands.spawn(AudioBundle {
                source,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    ..default()
                },
                ..default()
            });
        }
    }
}
