use bevy::color::palettes::css::ORANGE;

use crate::prelude::*;

pub struct RangeGizmosPlugin;

impl Plugin for RangeGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, config_line_width)
            .add_systems(
            Update,
            (
                draw_monster_hearing_ring_system,
                draw_player_bomb_picking_range,
            ),
        );
    }
}

fn config_line_width(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 0.3;
}

fn draw_monster_hearing_ring_system(
    mut gizmos: Gizmos,
    monsters_query: Query<(&Transform, &Monster), With<Monster>>,
) {
    for (transform, monster) in monsters_query.iter() {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            monster.hearing_ring_distance,
            monster.state.to_hearing_ring_gizmo_color(),
        );
    }
}

fn draw_player_bomb_picking_range(
    mut gizmos: Gizmos,
    player_query: Query<&Transform, With<Player>>,
) {
    for transform in player_query.iter() {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            PLAYER_BOMB_PICKING_RANGE,
            Color::from(ORANGE),
        );
    }
}
