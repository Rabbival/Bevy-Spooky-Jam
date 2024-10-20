use crate::prelude::*;

pub struct MonsterStateChangerPlugin;

impl Plugin for MonsterStateChangerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            if FunctionalityOverride::MonstersNeverAttackOrFlee.disabled() {
                (
                    update_monster_hearing_rings,
                    listen_for_monsters_done_spawning,
                )
                    .chain()
                    .in_set(MonsterSystemSet::StateChanging)
            } else {
                listen_for_monsters_done_spawning.in_set(MonsterSystemSet::StateChanging)
            },
        );
    }
}

fn listen_for_monsters_done_spawning(
    mut done_timers_listener: EventReader<TimerDoneEvent>,
    mut monster_state_set_writer: EventWriter<MonsterStateChanged>,
    mut monsters_query: Query<(&mut Monster, Entity)>,
    mut commands: Commands,
) {
    for done_timer in done_timers_listener.read() {
        if let TimerDoneEventType::DeclareSpawnDone = done_timer.event_type {
            for entity in done_timer.affected_entities.affected_entities_iter() {
                if let Ok((mut monster, monster_entity)) = monsters_query.get_mut(entity) {
                    monster_state_set_writer.send(MonsterStateChanged {
                        monster: entity,
                        next_state: MonsterState::default(),
                        previous_state: monster.state,
                    });
                    monster.state = MonsterState::default();
                    if FunctionalityOverride::DontCheckMonsterColliders.disabled() {
                        commands
                            .entity(monster_entity)
                            .insert(PlayerMonsterCollider::new(MONSTER_COLLIDER_RADIUS));
                    }
                }
            }
        }
    }
}

fn update_monster_hearing_rings(
    mut monster_state_set_writer: EventWriter<MonsterStateChanged>,
    mut monsters_query: Query<(&Transform, &mut Monster, Entity)>,
    player_query: Query<(&Transform, Entity), With<Player>>,
    bomb_query: Query<(&Transform, &Bomb, Entity)>,
) {
    for (monster_transform, mut monster, monster_entity) in &mut monsters_query {
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
            monster_state_set_writer.send(MonsterStateChanged {
                monster: monster_entity,
                next_state,
                previous_state: monster.state,
            });
            monster.state = next_state;
        }
    }
}

fn determine_most_danger_posing_bomb_location(
    monster_transform: &Transform,
    monster: &Monster,
    bomb_query: &Query<(&Transform, &Bomb, Entity)>,
) -> Option<Entity> {
    let mut most_dangerous_bomb_details: Option<(&Transform, &Bomb, Entity)> = None;
    for (bomb_transform, bomb, bomb_entity) in bomb_query {
        if let BombState::PreHeld = bomb.state {
            continue;
        }
        if bomb_transform
            .translation
            .distance(monster_transform.translation)
            < monster.hearing_ring_distance
        {
            match most_dangerous_bomb_details {
                Some((_, most_dangerous_bomb, _)) => {
                    if bomb.time_until_explosion < most_dangerous_bomb.time_until_explosion {
                        most_dangerous_bomb_details = Some((bomb_transform, bomb, bomb_entity));
                    }
                }
                None => most_dangerous_bomb_details = Some((bomb_transform, bomb, bomb_entity)),
            }
        }
    }
    most_dangerous_bomb_details.map(|(_, _, bomb_entity)| bomb_entity)
}
