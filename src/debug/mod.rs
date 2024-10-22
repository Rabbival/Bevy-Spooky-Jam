use crate::{prelude::*, read_no_field_variant};

pub mod consts;
pub mod enums;
pub mod gizmos;
pub mod print_config_struct;
pub mod print_log;
pub mod print_vec;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_for_debug_key_presses);
    }
}

fn listen_for_debug_key_presses(
    mut game_event_reader: EventReader<GameEvent>,
    entities_query: Query<Entity>,
) {
    for _debug_request in read_no_field_variant!(game_event_reader, GameEvent::DebugKeyPressed) {
        info!("{:?}", entities_query.iter().count());
    }
}
