use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct MonsterStateSetRequest {
    pub monster: Entity,
    pub next_state: MonsterState,
    pub previous_state: MonsterState,
}

pub struct MonsterStateSetRequestPlugin;

impl Plugin for MonsterStateSetRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MonsterStateSetRequest>();
    }
}
