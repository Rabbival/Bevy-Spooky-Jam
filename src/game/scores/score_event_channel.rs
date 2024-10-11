use crate::prelude::*;

#[derive(Event)]
pub struct UpdatePlayerScoreEvent {
    pub points: u32,
}

pub struct ScoreEventPlugin;

impl Plugin for ScoreEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<UpdatePlayerScoreEvent>();
    }
}
