use crate::prelude::*;

pub struct MonsterManagerPlugin;

impl Plugin for MonsterManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_monster_hearing_rings, /*TODO (Eitan): change path to flee/chase*/
            ),
        );
    }
}

fn update_monster_hearing_rings(
    mut monsters_query: Query<(&Transform, &mut Monster)>,
    player_query: Query<&Transform, With<Player>>,
    bomb_query: Query<(&Transform, &Bomb), With<Bomb>>,
) {
    for (monster_transform, mut monster) in monsters_query.iter_mut() {
        for player_transform in &player_query {
            if is_point_inside_ring(
                player_transform,
                monster_transform,
                monster.hearing_ring_distance,
            ) {
                monster.state = MonsterState::Chasing;
                //TODO: set chasing point
                continue;
            }
        }
        //TODO: maybe take outside to a smaller function
        let mut bomb_that_is_closest_to_explosion: Option<&Bomb> = None;
        for (bomb_transform, bomb) in &bomb_query {
            if is_point_inside_ring(
                bomb_transform,
                monster_transform,
                monster.hearing_ring_distance,
            ) {
                match bomb_that_is_closest_to_explosion {
                    Some(closest_explosion_bomb) => {
                        if bomb.currently_displayed < closest_explosion_bomb.currently_displayed {
                            bomb_that_is_closest_to_explosion = Some(bomb);
                        }
                    }
                    None => bomb_that_is_closest_to_explosion = Some(bomb),
                }
            }
        }
        match bomb_that_is_closest_to_explosion {
            Some(closest_to_explosion_bomb) => {
                monster.state = MonsterState::Fleeing;
                //TODO: Run in opposite direcion
            }
            None => monster.state = MonsterState::Idle,
        };
    }
}

fn is_point_inside_ring(point: &Transform, ring: &Transform, radius: f32) -> bool {
    let distance_x = (point.translation.x - ring.translation.x).powf(2.0);
    let distance_y = (point.translation.y - ring.translation.y).powf(2.0);
    distance_x + distance_y < radius.powf(2.0)
}
