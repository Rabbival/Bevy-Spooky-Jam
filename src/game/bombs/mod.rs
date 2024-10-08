use crate::prelude::*;

pub mod bomb;
pub mod bomb_error;
pub mod bomb_picking;
pub mod bomb_spawner;
pub mod bomb_spawning_sequence_manager;
pub mod bomb_state;
pub mod bomb_throwing;
pub mod consts;

pub struct BombsPlugin;

impl Plugin for BombsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BombSpawnerPlugin,
            BombSpawningSequenceManagerPlugin,
            BombPickingPlugin,
            BombThrowingPlugin,
        ));
    }
}
