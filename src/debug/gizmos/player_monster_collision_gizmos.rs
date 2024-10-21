use bevy::color::palettes::css::LIME;

use crate::prelude::*;

pub struct PlayerMonsterCollisionGizmosPlugin;

impl Plugin for PlayerMonsterCollisionGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_player_monster_collision_radius_preview);
    }
}

fn draw_player_monster_collision_radius_preview(
    mut gizmos: Gizmos,
    player_query: Query<(&GlobalTransform, &PlayerMonsterCollider), With<Player>>,
    monster_query: Query<(&GlobalTransform, &PlayerMonsterCollider), With<Monster>>,
) {
    for (transform, player_monster_collider) in &player_query {
        gizmos.circle_2d(
            Vec2::new(transform.translation().x, transform.translation().y),
            player_monster_collider.radius,
            Color::from(LIME),
        );
    }
    for (transform, player_monster_collider) in &monster_query {
        gizmos.circle_2d(
            Vec2::new(transform.translation().x, transform.translation().y),
            player_monster_collider.radius,
            Color::from(LIME),
        );
    }
}
