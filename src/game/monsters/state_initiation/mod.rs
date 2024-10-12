use crate::prelude::*;

use super::spawn_monster_move_calculator;

pub mod chase_state_initiation;
pub mod flee_state_initiation;
pub mod idle_state_initiation;

pub struct MonsterStateInitiationPlugin;

impl Plugin for MonsterStateInitiationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MonsterChaseStateInitiationPlugin,
            MonsterIdleStateInitiationPlugin,
            MonsterFleeStateInitiationPlugin,
        ));
    }
}

fn replace_monster_path(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    monster: &Monster,
    monster_entity: Entity,
    monster_location: Vec3,
    location_to_chase: Vec3,
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timer_parent_sequence_query: &Query<&TimerParentSequence, With<EmittingTimer>>,
    commands: &mut Commands,
) -> Option<TimerParentSequence> {
    let maybe_path_timer_parent_sequence = destroy_current_path_timer_and_calculator(
        monster,
        affecting_timer_calculators,
        emitting_timer_parent_sequence_query,
        commands,
    );
    if let Some(path_timer_parent_sequence) = maybe_path_timer_parent_sequence {
        let speed = if let MonsterState::Chasing(_) = monster.state {
            MONSTER_SPEED_WHEN_CHASING
        } else {
            MONSTER_SPEED_WHEN_FLEEING
        };
        let distance_to_goal = monster_location.distance(location_to_chase);
        let move_calculator =
            spawn_monster_move_calculator(monster_location, location_to_chase, commands);
        timer_fire_request_writer.send(TimerFireRequest {
            timer: EmittingTimer::new(
                vec![TimerAffectedEntity {
                    affected_entity: monster_entity,
                    value_calculator_entity: Some(move_calculator),
                }],
                vec![TimeMultiplierId::GameTimeMultiplier],
                distance_to_goal / speed,
                TimerDoneEventType::Nothing,
            ),
            parent_sequence: Some(path_timer_parent_sequence),
        });
    }
    maybe_path_timer_parent_sequence
}

fn destroy_current_path_timer_and_calculator(
    monster: &Monster,
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timer_parent_sequence_query: &Query<&TimerParentSequence, With<EmittingTimer>>,
    commands: &mut Commands,
) -> Option<TimerParentSequence> {
    if let Some(direct_line_movers) =
        affecting_timer_calculators.get(&TimerGoingEventType::Move(MovementType::InDirectLine))
    {
        for timer_and_calculator in direct_line_movers {
            if let Ok(parent_sequence) =
                emitting_timer_parent_sequence_query.get(timer_and_calculator.timer)
            {
                if monster.path_timer_sequence == parent_sequence.parent_sequence {
                    despawn_recursive_notify_on_fail(
                        timer_and_calculator.timer,
                        "timer when changing monster state",
                        commands,
                    );
                    return Some(*parent_sequence);
                }
            }
        }
    }
    None
}

fn visualize_calm_down(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    monster_entity: Entity,
    monster_scale: Vec3,
    monster_alpha: f32,
    commands: &mut Commands,
) {
    let alpha_calculator = spawn_monster_calm_down_alpha_set_calculator(monster_alpha, commands);
    let scale_calculator = spawn_monster_calm_down_scale_change_calculator(monster_scale, commands);
    timer_fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![
                TimerAffectedEntity {
                    affected_entity: monster_entity,
                    value_calculator_entity: Some(alpha_calculator),
                },
                TimerAffectedEntity {
                    affected_entity: monster_entity,
                    value_calculator_entity: Some(scale_calculator),
                },
            ],
            vec![TimeMultiplierId::GameTimeMultiplier],
            MONSTER_CHASE_START_VISUAL_CHANGE_DURATION,
            TimerDoneEventType::Nothing,
        ),
        parent_sequence: None,
    });
}

fn spawn_monster_calm_down_alpha_set_calculator(
    monster_current_alpha: f32,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::KeepNewTimer,
            ValueByInterpolation::from_goal_and_current(
                monster_current_alpha,
                MONSTER_FADED_ALPHA,
                Interpolator::new(2.0),
            ),
            TimerGoingEventType::SetAlpha,
        ))
        .id()
}

fn spawn_monster_calm_down_scale_change_calculator(
    monster_current_size: Vec3,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::KeepNewTimer,
            ValueByInterpolation::from_goal_and_current(
                monster_current_size,
                Vec3::ONE,
                Interpolator::new(2.0),
            ),
            TimerGoingEventType::Scale,
        ))
        .id()
}
