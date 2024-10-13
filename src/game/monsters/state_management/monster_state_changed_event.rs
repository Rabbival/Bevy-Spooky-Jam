use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct MonsterStateChanged {
    pub monster: Entity,
    pub next_state: MonsterState,
    pub previous_state: MonsterState,
}

pub struct MonsterStateChangedEventPlugin;

impl Plugin for MonsterStateChangedEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MonsterStateChanged>();
    }
}
