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
        match sound {
            SoundEvent::BombExplodeSoundEvent => {
                source = sound_assets_resource.bomb_explode.clone();
            }
            SoundEvent::BombPickUpEvent => {
                source = sound_assets_resource.bomb_pick_up.clone();
            }
            SoundEvent::BombThrowEvent => {
                source = sound_assets_resource.bomb_throw.clone();
            }
            SoundEvent::BombTickEvent => {
                source = sound_assets_resource.bomb_tick.clone();
            }
            SoundEvent::MonsterBattleCry => {
                source = sound_assets_resource.monster_battle_cry.clone();
            }
            SoundEvent::MonsterDeathCry => {
                source = sound_assets_resource.monster_death_cry.clone();
            }
        }
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
