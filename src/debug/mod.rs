use crate::{prelude::*, read_no_field_variant};

pub mod consts;
pub mod enums;
pub mod game_session_log;
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
    enemies_query: Query<Entity, With<Monster>>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
) {
    for _debug_request in read_no_field_variant!(game_event_reader, GameEvent::DebugKeyPressed) {
        let mut emitting_timers = vec![];
        for monster_entity in &enemies_query {
            emitting_timers.push(TimerAffectedEntity {
                affected_entity: monster_entity,
                value_calculator_entity: None,
            });
        }
        timer_fire_request_writer.send(TimerFireRequest {
            timer: EmittingTimer::new(
                emitting_timers,
                vec![],
                0.001,
                TimerDoneEventType::DespawnAffectedEntities(
                    DespawnPolicy::DespawnSelfAndAllThatAffectsIt,
                ),
            ),
            parent_sequence: None,
        });
    }
}
