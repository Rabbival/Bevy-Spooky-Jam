use crate::prelude::*;

pub struct MonsterStateChangerPlugin;

impl Plugin for MonsterStateChangerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_monster_hearing_rings,
            ),
        );
    }
}

fn update_monster_hearing_rings(
    mut monster_state_set_writer: EventWriter<MonserStateSetRequest>,
    monsters_query: Query<(&Transform, &Monster)>,
    player_query: Query<(&Transform, Entity), With<Player>>,
    bomb_query: Query<(&Transform, &Bomb)>,
) {
    'monsters_loop: for (monster_transform, monster) in
        &monsters_query
    {
        for (player_transform, player_entity) in &player_query {
            if is_point_inside_ring(
                player_transform,
                monster_transform,
                monster.hearing_ring_distance,
            ) {
                monster_state_set_writer.send(MonsterStateSetRequest(MonsterState::Chasing(player_entity)));
                continue 'monsters_loop;
            }
        }
        let maybe_most_danger_posing_bomb_location =
            determine_most_danger_posing_bomb_location(monster_transform, &monster, &bomb_query);
        match maybe_most_danger_posing_bomb_location {
            Some(most_danger_posing_bomb_location) => {
                monster_state_set_writer.send(MonsterState::Fleeing(most_danger_posing_bomb_location));
            }
            None => monster_state_set_writer.send(MonsterState::Idle);
        };
    }
}

fn determine_most_danger_posing_bomb_location(
    monster_transform: &Transform,
    monster: &Monster,
    bomb_query: &Query<(&Transform, &Bomb)>,
) -> Option<Vec3> {
    let mut most_dangerous_bomb_details: Option<(&Transform, &Bomb)> = None;
    for (bomb_transform, bomb) in bomb_query {
        if is_point_inside_ring(
            bomb_transform,
            monster_transform,
            monster.hearing_ring_distance,
        ) {
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

fn is_point_inside_ring(point: &Transform, ring: &Transform, radius: f32) -> bool {
    let distance_x = (point.translation.x - ring.translation.x).powf(2.0);
    let distance_y = (point.translation.y - ring.translation.y).powf(2.0);
    distance_x + distance_y < radius.powf(2.0)
}
