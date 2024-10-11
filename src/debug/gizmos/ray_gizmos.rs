use bevy::color::palettes::css::YELLOW;

use crate::prelude::*;

pub struct RayGizmosPlugin;

impl Plugin for RayGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_held_bomb_path_preview);
    }
}

fn draw_held_bomb_path_preview(
    mut gizmos: Gizmos,
    bomb_query: Query<(&GlobalTransform, &Bomb)>,
    cursor: Res<CursorWorldPosition>,
) {
    for (bomb_transform, bomb) in &bomb_query {
        if let BombState::Held = bomb.bomb_state {
            gizmos.line_2d(
                bomb_transform.translation().truncate(),
                cursor.0,
                Color::from(YELLOW),
            );
        }
    }
}
