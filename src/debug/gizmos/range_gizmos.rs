use crate::prelude::*;
use bevy::color::palettes::css::{ORANGE, RED, YELLOW};

pub struct RangeGizmosPlugin;

impl Plugin for RangeGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, config_line_width).add_systems(
            Update,
            (
                // draw_monster_hearing_ring_system,
                draw_player_bomb_picking_range,
                draw_bomb_explosion_radius,
                draw_explode_on_contact_radius,
            ),
        );
    }
}

fn config_line_width(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 0.3;
}

// fn draw_monster_hearing_ring_system(
//     mut gizmos: Gizmos,
//     monsters_query: Query<(&Transform, &Monster), With<Monster>>,
// ) {
//     for (transform, monster) in &monsters_query {
//         gizmos.circle_2d(
//             Vec2::new(transform.translation.x, transform.translation.y),
//             monster.hearing_ring_distance,
//             monster.state.to_hearing_ring_gizmo_color(),
//         );
//     }
// }

fn draw_player_bomb_picking_range(
    mut gizmos: Gizmos,
    player_query: Query<&Transform, With<Player>>,
) {
    for transform in &player_query {
        gizmos.circle_2d(
            Vec2::new(transform.translation.x, transform.translation.y),
            PLAYER_BOMB_PICKING_RANGE,
            Color::from(ORANGE),
        );
    }
}

fn draw_bomb_explosion_radius(
    mut gizmos: Gizmos,
    bomb_query: Query<(&Transform, &Bomb), With<Bomb>>,
) {
    for (transform, bomb) in &bomb_query {
        if let BombState::PostHeld = bomb.state {
            gizmos.circle_2d(
                Vec2::new(transform.translation.x, transform.translation.y),
                BOMB_EXPLOSION_RADIUS,
                Color::from(YELLOW),
            );
        }
    }
}

fn draw_explode_on_contact_radius(
    mut gizmos: Gizmos,
    explode_in_contact: Query<&ExplodeInContact>,
) {
    for circle in explode_in_contact
        .iter()
        .map(|component| component.bounding_circle)
    {
        gizmos.circle_2d(circle.center, circle.radius(), Color::from(RED));
    }
}
