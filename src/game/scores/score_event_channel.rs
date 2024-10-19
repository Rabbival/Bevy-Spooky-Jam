use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct AppendToPlayerScoreEvent(pub u32);

pub struct ScoreEventPlugin;

impl Plugin for ScoreEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AppendToPlayerScoreEvent>();
    }
}
