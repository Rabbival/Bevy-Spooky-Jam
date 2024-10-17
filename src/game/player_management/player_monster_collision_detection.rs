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
        )
        .add_systems(
            Update,
            listen_for_collider_radius_update_requests.in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn listen_for_collider_radius_update_requests(
    mut event_reader: EventReader<TimerGoingEvent<Vec3>>,
    mut monsters_collider_query: Query<&mut PlayerMonsterCollider>,
) {
    for event_from_timer in event_reader.read() {
        if let TimerGoingEventType::Scale = event_from_timer.event_type {
            if let Ok(mut monsters_collider) =
                monsters_collider_query.get_mut(event_from_timer.entity)
            {
                monsters_collider.radius +=
                    event_from_timer.value_delta.x * MONSTER_COLLIDER_RADIUS_FACTOR_WHEN_CHASING;
            }
        }
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
    mut game_event_writer: EventWriter<GameEvent>,
    query: Query<(Entity, &PlayerMonsterCollider), With<Player>>,
    mut commands: Commands,
) {
    for (_entity, collider) in query.iter() {
        for &collided_entity in collider.colliding_monsters.iter() {
            // monster with another monster
            if query.get(collided_entity).is_ok() {
                continue;
            }
            commands.entity(_entity).despawn();
            game_event_writer.send(GameEvent::RestartGame);
        }
    }
}
