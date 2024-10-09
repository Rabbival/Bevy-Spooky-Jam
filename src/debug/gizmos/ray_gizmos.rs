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
    player_query: Query<(&Transform, &FacingDirection), With<Player>>,
) {
    for (bomb_transform, bomb) in &bomb_query {
        if let BombState::Held = bomb.bomb_state {
            for (player_transform, facing_direction) in &player_query {
                let bomb_destination =
                    player_transform.translation + facing_direction.0 * BOMB_THROWING_DISTANCE;
                gizmos.line_2d(
                    bomb_transform.translation().truncate(),
                    bomb_destination.truncate(),
                    Color::from(YELLOW),
                );
            }
        }
    }
}
