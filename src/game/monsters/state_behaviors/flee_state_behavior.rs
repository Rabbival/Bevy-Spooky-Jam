use crate::prelude::*;

pub struct MonsterFleeStateBehaviorPlugin;

impl Plugin for MonsterFleeStateBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_fleeing_monsters.in_set(MonsterSystemSet::FleeingUpdating),
        );
    }
}

fn update_fleeing_monsters() {}
