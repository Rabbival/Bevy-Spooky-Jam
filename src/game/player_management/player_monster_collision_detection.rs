use crate::prelude::*;

pub struct PlayerMonsterCollisionDetectionPlugin;

impl Plugin for PlayerMonsterCollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, dummy_system);
    }
}

fn dummy_system() {

}
