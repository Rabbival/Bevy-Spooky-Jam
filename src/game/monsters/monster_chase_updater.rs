use crate::prelude::*;

use super::spawn_monster_move_calculator;

pub struct MonsterChaseUpdaterPlugin;

impl Plugin for MonsterChaseUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_chasing_monsters.in_set(MonsterSystemSet::ChasingUpdating),
        );
    }
}

fn update_chasing_monsters(
    just_changed_transforms: Query<&Transform, Changed<Transform>>,
    monsters_query: Query<(Entity, &Monster, &Transform, &AffectingTimerCalculators)>,
    mut emitting_timers_with_parents_query: Query<(&mut EmittingTimer, &TimerParentSequence)>,
    mut commands: Commands,
) {
    for (monster_entity, monster, monster_transform, affecting_timer_calculators) in &monsters_query
    {
        if let MonsterState::Chasing(entity_chasing) = monster.state {
            if let Ok(transform) = just_changed_transforms.get(entity_chasing) {
                let new_chase_path_calculator = spawn_monster_move_calculator(
                    monster_transform.translation,
                    transform.translation,
                    &mut commands,
                );
                reset_chase_timer_with_new_calculator(
                    TimerAffectedEntity {
                        affected_entity: monster_entity,
                        value_calculator_entity: Some(new_chase_path_calculator),
                    },
                    monster_transform
                        .translation
                        .distance(transform.translation)
                        / MONSTER_SPEED_WHEN_CHASING,
                    monster,
                    affecting_timer_calculators,
                    &mut emitting_timers_with_parents_query,
                );
            }
        }
    }
}

fn reset_chase_timer_with_new_calculator(
    new_affected_entity: TimerAffectedEntity,
    new_timer_duration: f32,
    monster: &Monster,
    affecting_timer_calculators: &AffectingTimerCalculators,
    emitting_timers_with_parents_query: &mut Query<(&mut EmittingTimer, &TimerParentSequence)>,
) {
    if let Some(direct_line_movers) =
        affecting_timer_calculators.get(&TimerGoingEventType::Move(MovementType::InDirectLine))
    {
        for timer_and_calculator in direct_line_movers {
            if let Ok((mut timer, parent_sequence)) =
                emitting_timers_with_parents_query.get_mut(timer_and_calculator.timer)
            {
                if monster.path_timer_sequence == parent_sequence.parent_sequence {
                    if !timer.affected_entities.is_empty() {
                        timer.affected_entities.array[0] = Some(new_affected_entity);
                        timer.reset();
                        timer.duration = new_timer_duration;
                        print_info(
                            &format!("Updated chasing path for monster: {:?}", monster),
                            vec![LogCategory::Monster],
                        );
                    }
                }
            }
        }
    }
}
