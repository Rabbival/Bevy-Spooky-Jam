use bevy::audio::Volume;

use crate::prelude::*;

pub struct MusicPlayerPlugin;

impl Plugin for MusicPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_and_play_music)
            .add_systems(Update, temp_test_system);
    }
}

fn load_and_play_music(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music/music_calm_layer.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::new(1.0)),
        },
        MusicLayer(1),
    ));
    commands.spawn((
        AudioBundle {
            source: asset_server.load("music/music_intense_layer.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::ZERO),
        },
        MusicLayer(2),
    ));
}

fn temp_test_system(
    mut monster_state_set_listener: EventReader<MonsterStateSetRequest>,
    monsters: Query<(Entity, &Monster)>,
    query: Query<(&MusicLayer, &AudioSink)>,
) {
    'request_loop: for set_request in monster_state_set_listener.read() {
        if let MonsterState::Chasing(_) = set_request.next_state {
            for (music_layer, audio) in &query {
                if music_layer.0 == 2 {
                    audio.set_volume(1.0); //use timers of course to make it happen gradually, I'd use TakeNewTimer policy
                }
            }
        } else {
            for (entity, monster) in &monsters {
                if entity != set_request.monster {
                    if let MonsterState::Chasing(_) = monster.state {
                        continue 'request_loop;
                    }
                }
            }
            //if we got here that means no monster is chasing the player
            for (music_layer, audio) in &query {
                if music_layer.0 == 2 {
                    audio.set_volume(0.0);
                }
            }
        }
    }
}
