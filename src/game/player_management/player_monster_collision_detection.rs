use crate::prelude::*;

pub struct PlayerMonsterCollisionDetectionPlugin;

impl Plugin for PlayerMonsterCollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_monster_collision_detection_system,
                handle_player_monster_collisions,
            ),
        );
    }
}

fn player_monster_collision_detection_system(
    mut query: Query<(Entity, &GlobalTransform, &mut PlayerMonsterCollider)>,
) {
    let mut colliding_monsters: HashMap<Entity, Vec<Entity>> = HashMap::new();
    // first phase: detect collisions
    for (entity_a, transform_a, collider_a) in query.iter() {
        for (entity_b, transform_b, collider_b) in query.iter() {
            if entity_a != entity_b {
                let distance = transform_a
                    .translation()
                    .distance(transform_b.translation());
                if distance < collider_a.radius + collider_b.radius {
                    colliding_monsters
                        .entry(entity_a)
                        .or_insert_with(Vec::new)
                        .push(entity_b);
                }
            }
        }
    }
    // second phase: update colliders
    for (entity, _, mut collider) in query.iter_mut() {
        collider.colliding_monsters.clear();
        if let Some(collisions) = colliding_monsters.get(&entity) {
            collider
                .colliding_monsters
                .extend(collisions.iter().copied());
        }
    }
}

fn handle_player_monster_collisions(
    //mut commands: Commands,
    query: Query<(Entity, &PlayerMonsterCollider), With<Monster>>,
) {
    for (_entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_monsters.iter() {
            // monster with another monster
            if query.get(collided_entity).is_ok() {
                continue;
            }
            // monster with player
            info!("monster with player collision");
            // TODO spawn GAME OVER text
            // TODO stop stopwatch
            // TODO stop monsters & threw bombs
        }
    }
}
