use crate::prelude::*;

pub mod consts;
pub mod monster;
pub mod monster_error;
pub mod monster_manager;
pub mod monster_spawner;
pub mod monster_spawning_sequence_manager;
pub mod monster_state;

pub struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MonsterSpawnerPlugin,
            MonsterSpawningSequenceManagerPlugin,
            MonsterManagerPlugin,
        ));
    }
}
