use crate::prelude::*;
use crate::prelude::monster_spawner::MonsterSpawnerPlugin;
use crate::prelude::monster_spawning_sequence_manager::MonsterSpawningSequenceManagerPlugin;

pub mod consts;
pub mod monster_spawner;
pub mod monster_state;
pub mod monster_spawning_sequence_manager;
pub mod monster_error;

pub struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MonsterSpawnerPlugin, MonsterSpawningSequenceManagerPlugin));
    }
}
