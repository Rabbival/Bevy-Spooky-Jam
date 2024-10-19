use crate::prelude::*;

pub mod components;
pub mod score_event_channel;
pub mod score_manager;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ScoreManagerPlugin, ScoreEventPlugin));
    }
}
