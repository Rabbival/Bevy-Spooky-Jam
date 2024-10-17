use crate::prelude::*;

pub struct RespawnerPlugin;

impl Plugin for RespawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            show_again_and_respawn_world.in_set(GameRestartSystemSet::RespawnerCall),
        );
    }
}

fn show_again_and_respawn_world(mut game_over_listener: EventReader<GameOver>) {
    //Fade "again" in then restart then fade it out
}
