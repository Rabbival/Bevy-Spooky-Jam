use crate::prelude::*;
use bevy::audio::{PlaybackMode, Volume};

pub struct SoundPlayerPlugin;

impl Plugin for SoundPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                bombs_and_monsters_sounds_event_listener,
                update_sound_resource_speed,
            ),
        );
    }
}

fn update_sound_resource_speed(
    mut multiplier_change_reader: EventReader<TimerGoingEvent<f32>>,
    mut sound_assets_resource: ResMut<SoundAssets>,
    time_multipliers: Query<&TimeMultiplier>,
) {
    for event in multiplier_change_reader.read() {
        if let TimerGoingEventType::ChangeTimeMultiplierSpeed = event.event_type {
            if let Ok(multiplier) = time_multipliers.get(event.entity) {
                if let SOUND_TIME_MULTIPLIER_ID = multiplier.id() {
                    sound_assets_resource.sound_speed += event.value_delta;
                }
            }
        }
    }
}

fn bombs_and_monsters_sounds_event_listener(
    sound_assets_resource: Res<SoundAssets>,
    mut sound_events_reader: EventReader<SoundEvent>,
    mut commands: Commands,
) {
    for sound in sound_events_reader.read() {
        let source;
        let mut volume_override = None;
        match sound {
            SoundEvent::BombExplodeSoundEvent => {
                source = sound_assets_resource.bomb_explode.clone();
                volume_override = Some(0.85);
            }
            SoundEvent::BombPickUpEvent => {
                source = sound_assets_resource.bomb_pick_up.clone();
            }
            SoundEvent::BombThrowEvent => {
                source = sound_assets_resource.bomb_throw.clone();
                volume_override = Some(0.5);
            }
            SoundEvent::BombTickEvent(volume) => {
                source = sound_assets_resource.bomb_tick.clone();
                volume_override = Some(*volume);
            }
            SoundEvent::MonsterBattleCry => {
                source = sound_assets_resource.monster_battle_cry.clone();
            }
            SoundEvent::MonsterDeathCry => {
                source = sound_assets_resource.monster_death_cry.clone();
            }
        }
        commands.spawn((
            AudioBundle {
                source,
                settings: PlaybackSettings {
                    mode: PlaybackMode::Despawn,
                    volume: Volume::new(volume_override.unwrap_or(1.0)),
                    speed: sound_assets_resource.sound_speed,
                    ..default()
                },
                ..default()
            },
            AffectingTimeMultiplier(SOUND_TIME_MULTIPLIER_ID),
        ));
    }
}
