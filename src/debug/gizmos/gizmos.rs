use crate::prelude::*;
use bevy::color::palettes::css::*;

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_monster_hearing_ring_system);
    }
}

fn draw_monster_hearing_ring_system(
    mut gizmos: Gizmos,
    monsters_query: Query<(&Transform, &Monster), With<Monster>>,
) {
    for (transform, monster) in monsters_query.iter() {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            monster.hearing_ring_distance,
            NAVY,
        );
    }
}
