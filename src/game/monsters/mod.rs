use crate::prelude::*;

pub mod consts;
pub mod monster;
pub mod monster_chase_updater;
pub mod monster_error;
pub mod monster_listening;
pub mod monster_spawner;
pub mod monster_spawning_sequence_manager;
pub mod monster_state;
pub mod monster_state_set_request;
pub mod state_initiation;

pub struct MonstersPlugin;

impl Plugin for MonstersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MonsterSpawnerPlugin,
            MonsterSpawningSequenceManagerPlugin,
            MonsterStateSetRequestPlugin,
            MonsterListeningPlugin,
            MonsterStateInitiationPlugin,
            MonsterChaseUpdaterPlugin,
        ));
    }
}

fn spawn_monster_chase_move_calculator(
    current_location: Vec3,
    location_to_chase: Vec3,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::AppendToTimersOfType,
            ValueByInterpolation::from_goal_and_current(
                current_location,
                location_to_chase,
                Interpolator::new(1.0),
            ),
            TimerGoingEventType::Move(MovementType::InDirectLine),
        ))
        .id()
}
