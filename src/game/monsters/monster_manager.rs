use crate::prelude::*;

pub struct MonsterManagerPlugin;

impl Plugin for MonsterManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_monster_hearing_rings);
    }
}

fn update_monster_hearing_rings(
    mut monsters_query: Query<(&Transform, &mut Monster)>,
    surrounding_objects_query: Query<(&Transform, Option<&Bomb>, Option<&Player>)>,
) {
    for (monster_transform, mut monster) in monsters_query.iter_mut() {
        for (surrounding_object_transform, maybe_bomb, maybe_player) in
            surrounding_objects_query.iter()
        {
            if is_point_inside_ring(
                surrounding_object_transform,
                monster_transform,
                monster.hearing_ring_distance,
            ) {
                if maybe_player.is_some() {
                    monster.state = MonsterState::Chasing;
                    monster.last_player_location_seen = surrounding_object_transform.translation;
                    break;
                }
                if maybe_bomb.is_some() {
                    monster.state = MonsterState::Fleeing;
                    // TODO fix this, it can be a memory leak!
                    monster
                        .bombs_location_seen
                        .push(surrounding_object_transform.translation);
                    break;
                }
            }
            monster.state = MonsterState::Idle;
        }
    }
}

fn is_point_inside_ring(point: &Transform, ring: &Transform, radius: f32) -> bool {
    let distance_x = (point.translation.x - ring.translation.x).powf(2.0);
    let distance_y = (point.translation.y - ring.translation.y).powf(2.0);
    distance_x + distance_y < radius.powf(2.0)
}
