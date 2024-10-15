use crate::prelude::*;

pub mod animation_starter;
pub mod state_change_visualizer;

pub struct MonsterVisualsPlugin;

impl Plugin for MonsterVisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MonsterStateChangeVisualizerPlugin,
            // MonsterAnimationStarterPlugin,
        ));
    }
}
