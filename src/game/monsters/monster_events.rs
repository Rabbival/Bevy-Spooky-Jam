use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct MonsterStateChanged {
    pub monster: Entity,
    pub next_state: MonsterState,
    pub previous_state: MonsterState,
}

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct MonsterStrayPathUpdated {
    pub monster_entity: Entity,
    pub new_delta: Vec2,
}

pub struct MonsterEventsPlugin;

impl Plugin for MonsterEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MonsterStateChanged>()
            .add_event::<MonsterStrayPathUpdated>();
    }
}
