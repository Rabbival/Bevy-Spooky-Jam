use crate::prelude::*;
use bevy::color::palettes::css::*;

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_monster_hearing_ring_system);
    }
}

fn draw_monster_hearing_ring_system(
    monsters_query: Query<&Transform, With<Monster>>,
    mut gizmos: Gizmos,
) {
    for monster in monsters_query.iter() {
        gizmos.circle_2d(
            Vec2::new(monster.translation.x, monster.translation.y),
            1.0,
            NAVY,
        );
    }
}
