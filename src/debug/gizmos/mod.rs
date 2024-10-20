use crate::prelude::*;

pub mod player_monster_collision_gizmos;
pub mod range_gizmos;
pub mod ray_gizmos;

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            #[cfg(debug_assertions)]
            PlayerMonsterCollisionGizmosPlugin,
            #[cfg(debug_assertions)]
            RangeGizmosPlugin,
            RayGizmosPlugin,
        ))
        .add_systems(Startup, config_line_width);
    }
}

fn config_line_width(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 0.3;
}
