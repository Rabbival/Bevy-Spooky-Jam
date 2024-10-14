use crate::prelude::*;

pub mod monster_state;
pub mod monster_state_changed_event;
pub mod monster_state_changer;

pub struct MonsterStateManagementPlugin;

impl Plugin for MonsterStateManagementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MonsterStateChangedEventPlugin, MonsterStateChangerPlugin));
    }
}
