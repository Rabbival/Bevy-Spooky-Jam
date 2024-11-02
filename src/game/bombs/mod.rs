use crate::prelude::*;

pub mod bomb;
pub mod bomb_error;
pub mod bomb_events;
pub mod bomb_explosion_previewer;
pub mod bomb_picking;
pub mod bomb_spawner;
pub mod bomb_spawning_sequence_manager;
pub mod bomb_state;
pub mod bomb_throwing;
pub mod bomb_ticker;
pub mod consts;
pub mod explode_in_contact_manager;
pub mod explosion_manager;
pub mod tags;

pub struct BombsPlugin;

impl Plugin for BombsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            BombSpawnerPlugin,
            BombSpawningSequenceManagerPlugin,
            BombPickingPlugin,
            BombThrowingPlugin,
            ExplosionManagerPlugin,
            BombTickerPlugin,
            BombEventsPlugin,
            BombExplosionPreviewerPlugin,
            ExplodeInContactManagerPlugin,
        ));
    }
}
