use crate::prelude::*;

use super::{replace_monster_path, visualize_calm_down};

pub struct MonsterFleeStateInitiationPlugin;

impl Plugin for MonsterFleeStateInitiationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_change_to_flee_requests.in_set(MonsterSystemSet::StateChanging),
        );
    }
}

fn listen_for_change_to_flee_requests(
    mut monster_state_set_listener: EventReader<MonsterStateSetRequest>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut monsters_query: Query<(
        &mut Monster,
        &Transform,
        &Handle<ColorMaterial>,
        Entity,
        &AffectingTimerCalculators,
    )>,
    emitting_timer_parent_sequence_query: Query<&TimerParentSequence, With<EmittingTimer>>,
    assets: Res<Assets<ColorMaterial>>,
    mut commands: Commands,
) {
    for request in monster_state_set_listener.read() {
        if let MonsterState::Fleeing(location_fleeing) = request.next_state {
            match monsters_query.get_mut(request.monster) {
                Ok((
                    mut monster,
                    monster_transform,
                    monster_color_handle,
                    monster_entity,
                    affecting_timer_calculators,
                )) => {
                    monster.state = request.next_state;
                    let location_to_flee_to =
                        (monster_transform.translation - location_fleeing) * 10.0;
                    if replace_monster_path(
                        &mut timer_fire_request_writer,
                        &monster,
                        monster_entity,
                        monster_transform.translation,
                        location_to_flee_to,
                        affecting_timer_calculators,
                        &emitting_timer_parent_sequence_query,
                        &mut commands,
                    )
                    .is_none()
                    {
                        print_error(
                            MonsterError::NoPathSequenceFoundOnStateChange(request.next_state),
                            vec![LogCategory::RequestNotFulfilled, LogCategory::Monster],
                        );
                    }
                    if let Some(monster_color) = assets.get(monster_color_handle.id()) {
                        if let MonsterState::Chasing(_) = request.previous_state {
                            visualize_calm_down(
                                &mut timer_fire_request_writer,
                                monster_entity,
                                monster_transform.scale,
                                monster_color.color.alpha(),
                                &mut commands,
                            );
                        }
                    }
                }
                Err(_) => {
                    print_error(
                        EntityError::EntityNotInQuery("monster when asked to initiate idle state"),
                        vec![LogCategory::RequestNotFulfilled, LogCategory::Monster],
                    );
                }
            }
        }
    }
}
