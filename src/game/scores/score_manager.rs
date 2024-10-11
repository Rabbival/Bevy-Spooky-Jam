use crate::game::scores::score_event_channel::UpdatePlayerScoreEvent;
use crate::prelude::*;

pub struct ScoreManagerPlugin;

impl Plugin for ScoreManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_player_scoring);
    }
}

fn update_player_scoring(
    mut players_query: Query<&mut Player>,
    mut events_reader: EventReader<UpdatePlayerScoreEvent>,
) {
    for mut player in players_query.iter_mut() {
        for event in events_reader.read() {
            player.score += event.points;
        }
    }
}
