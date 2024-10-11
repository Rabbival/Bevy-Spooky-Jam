use crate::game::scores::score_event_channel::ScoreEventPlugin;
use crate::game::scores::score_manager::ScoreManagerPlugin;
use crate::prelude::*;

pub mod score_event_channel;
pub mod score_manager;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ScoreManagerPlugin, ScoreEventPlugin));
    }
}
