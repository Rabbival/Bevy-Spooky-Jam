use crate::prelude::*;

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, config_line_width)
            .add_systems(Update, draw_monster_hearing_ring_system);
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
