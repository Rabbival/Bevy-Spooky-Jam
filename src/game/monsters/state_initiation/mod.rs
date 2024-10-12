use crate::prelude::*;

pub mod chase_state_initiation;
pub mod idle_state_initiation;

pub struct MonsterStateInitiationPlugin;

impl Plugin for MonsterStateInitiationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MonsterChaseStateInitiationPlugin,
            MonsterIdleStateInitiationPlugin,
        ));
    }
}
