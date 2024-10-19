use crate::prelude::*;

pub mod consts;
pub mod monster;
pub mod monster_audio;
pub mod monster_error;
pub mod monster_events;
pub mod monster_path;
pub mod monster_spawner;
pub mod monster_spawning_sequence_manager;
pub mod state_management;
pub mod visuals;

pub struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MonsterSpawnerPlugin,
            MonsterSpawningSequenceManagerPlugin,
            MonsterStateManagementPlugin,
            MonsterVisualsPlugin,
            MonsterPathUpdatingPlugin,
            MonsterAudioPlugin,
            MonsterEventsPlugin,
        ));
    }
}
