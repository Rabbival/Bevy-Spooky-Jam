use crate::prelude::*;

pub mod chase_state_behavior;
pub mod flee_state_behavior;

pub struct MonsterStateBehaviorsPlugin;

impl Plugin for MonsterStateBehaviorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MonsterChaseStateBehaviorPlugin,
            MonsterFleeStateBehaviorPlugin,
        ));
    }
}
