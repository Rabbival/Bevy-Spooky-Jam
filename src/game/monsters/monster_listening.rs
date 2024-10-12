use crate::prelude::*;

pub struct MonsterListeningPlugin;

impl Plugin for MonsterListeningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_monster_hearing_rings,
                listen_for_monsters_done_spawning,
            )
                .chain()
                .in_set(MonsterSystemSet::EnvironmentChecking),
        );
    }
}

fn listen_for_monsters_done_spawning(
    mut done_timers_listener: EventReader<TimerDoneEvent>,
    mut timer_fire_writer: EventWriter<TimerFireRequest>,
    mut monsters_query: Query<&mut Monster>,
    timer_sequence_query: Query<&TimerSequence>,
) {
    for done_timer in done_timers_listener.read() {
        if let TimerDoneEventType::DeclareSpawnDone = done_timer.event_type {
            for entity in done_timer.affected_entities.affected_entities_iter() {
                if let Ok(mut monster) = monsters_query.get_mut(entity) {
                    monster.state = MonsterState::default();
                    if let Ok(timer_sequence) =
                        timer_sequence_query.get(monster.path_timer_sequence)
                    {
                        if let Err(sequence_error) = timer_sequence
                            .fire_first_timer(monster.path_timer_sequence, &mut timer_fire_writer)
                        {
                            print_error(sequence_error, vec![LogCategory::RequestNotFulfilled]);
                        }
                    } else {
                        print_error(
                            EntityError::EntityNotInQuery(
                                "monster path sequence when monster done spawning",
                            ),
                            vec![LogCategory::Monster, LogCategory::RequestNotFulfilled],
                        );
                    }
                }
            }
        }
    }
}

fn update_monster_hearing_rings(
    mut monster_state_set_writer: EventWriter<MonsterStateSetRequest>,
    monsters_query: Query<(&Transform, &Monster, Entity)>,
    player_query: Query<(&Transform, Entity), With<Player>>,
    bomb_query: Query<(&Transform, &Bomb)>,
) {
    for (monster_transform, monster, monster_entity) in &monsters_query {
        if let MonsterState::Spawning = monster.state {
            continue;
        }
        let mut next_state = MonsterState::default();
        for (player_transform, player_entity) in &player_query {
            if player_transform
                .translation
                .distance(monster_transform.translation)
                < monster.hearing_ring_distance
            {
                next_state = MonsterState::Chasing(player_entity);
            }
        }
        if next_state == MonsterState::default() {
            let maybe_most_danger_posing_bomb_location = determine_most_danger_posing_bomb_location(
                monster_transform,
                &monster,
                &bomb_query,
            );
            match maybe_most_danger_posing_bomb_location {
                Some(most_danger_posing_bomb_location) => {
                    next_state = MonsterState::Fleeing(most_danger_posing_bomb_location);
                }
                None => {
                    next_state = MonsterState::Idle;
                }
            }
        }
        if next_state != monster.state {
            monster_state_set_writer.send(MonsterStateSetRequest {
                monster: monster_entity,
                next_state,
                previous_state: monster.state,
            });
        }
    }
}

fn determine_most_danger_posing_bomb_location(
    monster_transform: &Transform,
    monster: &Monster,
    bomb_query: &Query<(&Transform, &Bomb)>,
) -> Option<Vec3> {
    let mut most_dangerous_bomb_details: Option<(&Transform, &Bomb)> = None;
    for (bomb_transform, bomb) in bomb_query {
        if let BombState::PreHeld = bomb.bomb_state {
            continue;
        }
        if bomb_transform
            .translation
            .distance(monster_transform.translation)
            < monster.hearing_ring_distance
        {
            match most_dangerous_bomb_details {
                Some((_, most_dangerous_bomb)) => {
                    if bomb.time_until_explosion < most_dangerous_bomb.time_until_explosion {
                        most_dangerous_bomb_details = Some((bomb_transform, bomb));
                    }
                }
                None => most_dangerous_bomb_details = Some((bomb_transform, bomb)),
            }
        }
    }
    most_dangerous_bomb_details.map(|(transform, _)| transform.translation)
}
