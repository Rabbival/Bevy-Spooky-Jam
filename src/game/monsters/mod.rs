use crate::prelude::*;
use crate::prelude::monster_spawner::MonsterSpawnerPlugin;

pub mod consts;
pub mod monster_spawner;
pub mod monster_state;

pub struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MonsterSpawnerPlugin);
    }
}
