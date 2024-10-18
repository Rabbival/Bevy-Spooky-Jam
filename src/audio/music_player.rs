use bevy::audio::Volume;

use crate::{prelude::*, read_no_field_variant};

pub struct MusicPlayerPlugin;

impl Plugin for MusicPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_and_play_music).add_systems(
            Update,
            (
                change_layers_by_danger,
                listen_to_music_volume_update_requests,
                check_dangers_after_despawns,
                calm_down_music_when_game_over.in_set(GameRestartSystemSet::Spawning),
            ),
        );
    }
}

fn load_and_play_music(music_assets_resource: Res<MusicAssets>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: music_assets_resource.calm_layer_handle.clone(),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new(1.0)),
        },
        MusicLayer(1),
        AffectingTimeMultiplier(TimeMultiplierId::GameTimeMultiplier),
    ));
    commands.spawn((
        AudioBundle {
            source: music_assets_resource.intense_layer_handle.clone(),
            settings: PlaybackSettings::LOOP.with_volume(Volume::ZERO),
        },
        MusicLayer(2),
        AffectingTimeMultiplier(TimeMultiplierId::GameTimeMultiplier),
        AffectingTimerCalculators::default(),
    ));
}

fn change_layers_by_danger(
    mut monster_state_set_listener: EventReader<MonsterStateChanged>,
    mut fire_request_writer: EventWriter<TimerFireRequest>,
    monsters: Query<&Monster>,
    music_layers_query: Query<(Entity, &MusicLayer, &AudioSink)>,
    mut commands: Commands,
) {
    for set_request in monster_state_set_listener.read() {
        if let MonsterState::Chasing(_) = set_request.next_state {
            for (audio_entity, music_layer, audio) in &music_layers_query {
                if music_layer.0 == 2 {
                    fire_music_set_timer(
                        audio.volume(),
                        1.0,
                        audio_entity,
                        &mut fire_request_writer,
                        &mut commands,
                    );
                }
            }
        } else {
            calm_down_music_if_there_are_no_chasing_monsters(
                &mut fire_request_writer,
                &monsters,
                &music_layers_query,
                &mut commands,
            );
        }
    }
}

fn check_dangers_after_despawns(
    mut done_timers: EventReader<TimerDoneEvent>,
    mut fire_request_writer: EventWriter<TimerFireRequest>,
    monsters: Query<&Monster>,
    music_layers_query: Query<(Entity, &MusicLayer, &AudioSink)>,
    mut commands: Commands,
) {
    for done_event in done_timers.read() {
        if let TimerDoneEventType::DespawnAffectedEntities(_) = done_event.event_type {
            calm_down_music_if_there_are_no_chasing_monsters(
                &mut fire_request_writer,
                &monsters,
                &music_layers_query,
                &mut commands,
            );
        }
    }
}

fn calm_down_music_when_game_over(
    mut game_event_listener: EventReader<GameEvent>,
    mut fire_request_writer: EventWriter<TimerFireRequest>,
    music_layers_query: Query<(Entity, &MusicLayer, &AudioSink)>,
    mut commands: Commands,
) {
    for _game_over in read_no_field_variant!(game_event_listener, GameEvent::RestartGame) {
        for (audio_entity, music_layer, audio) in &music_layers_query {
            if music_layer.0 == 2 {
                fire_music_set_timer(
                    audio.volume(),
                    0.0,
                    audio_entity,
                    &mut fire_request_writer,
                    &mut commands,
                );
            }
        }
    }
}

fn calm_down_music_if_there_are_no_chasing_monsters(
    fire_request_writer: &mut EventWriter<TimerFireRequest>,
    monsters: &Query<&Monster>,
    music_layers_query: &Query<(Entity, &MusicLayer, &AudioSink)>,
    commands: &mut Commands,
) {
    for monster in monsters {
        if let MonsterState::Chasing(_) = monster.state {
            return;
        }
    }
    for (audio_entity, music_layer, audio) in music_layers_query {
        if music_layer.0 == 2 {
            fire_music_set_timer(
                audio.volume(),
                0.0,
                audio_entity,
                fire_request_writer,
                commands,
            );
        }
    }
}

fn fire_music_set_timer(
    current_volume: f32,
    wanted_volume: f32,
    audio_sink_entity: Entity,
    fire_request_writer: &mut EventWriter<TimerFireRequest>,
    commands: &mut Commands,
) {
    let calculator = spawn_volume_set_calculator(current_volume, wanted_volume, commands);
    fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: audio_sink_entity,
                value_calculator_entity: Some(calculator),
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            if wanted_volume > current_volume {
                FADE_IN_TIME
            } else {
                FADE_OUT_TIME
            },
            TimerDoneEventType::Nothing,
        ),
        parent_sequence: None,
    });
}

fn spawn_volume_set_calculator(
    current_volume: f32,
    wanted_volume: f32,
    commands: &mut Commands,
) -> Entity {
    let interpolator_power = if wanted_volume > current_volume {
        FADE_IN_POWER
    } else {
        FADE_OUT_POWER
    };
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::KeepNewTimer,
            ValueByInterpolation::from_goal_and_current(
                current_volume,
                wanted_volume,
                Interpolator::new(interpolator_power),
            ),
            TimerGoingEventType::SetMusicVolume,
        ))
        .id()
}

fn listen_to_music_volume_update_requests(
    mut event_reader: EventReader<TimerGoingEvent<f32>>,
    query: Query<&AudioSink>,
) {
    for event in event_reader.read() {
        if let TimerGoingEventType::SetMusicVolume = event.event_type {
            if let Ok(audio) = query.get(event.entity) {
                audio.set_volume(audio.volume() + event.value_delta);
            }
        }
    }
}
