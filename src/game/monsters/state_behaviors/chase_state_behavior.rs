use crate::prelude::*;

pub struct MonsterChaseStateBehaviorPlugin;

impl Plugin for MonsterChaseStateBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_chasing_monsters.in_set(MonsterSystemSet::ChasingUpdating),
        );
    }
}

fn update_chasing_monsters(monsters_query: Query<&Monster>) {}
