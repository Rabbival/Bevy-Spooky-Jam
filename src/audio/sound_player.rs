use crate::prelude::*;
use bevy::audio::PlaybackMode;

pub struct SoundPlayerPlugin;

impl Plugin for SoundPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bomb_explode);
    }
}

fn bomb_explode(
    sound_assets_resource: Res<SoundAssets>,
    mut events_reader: EventReader<BombExplodeSoundEvent>,
    mut commands: Commands,
) {
    for _event in events_reader.read() {
        commands.spawn(AudioBundle {
            source: sound_assets_resource.bomb_explode.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Despawn,
                ..default()
            },
            ..default()
        });
    }
}
