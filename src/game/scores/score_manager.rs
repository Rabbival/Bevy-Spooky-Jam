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
    mut world_championship_leaderboard_scoring_query: Query<
        &mut WorldChampionshipLeaderboardScoring,
    >,
    mut events_reader: EventReader<UpdatePlayerScoreEvent>,
) {
    let Ok(mut world_championship_leaderboard_scoring) =
        world_championship_leaderboard_scoring_query.get_single_mut()
    else {
        return;
    };
    for mut player in players_query.iter_mut() {
        for event in events_reader.read() {
            player.score += event.points;
            if world_championship_leaderboard_scoring.hi_score < player.score {
                world_championship_leaderboard_scoring.hi_score = player.score;
            }
        }
    }
}
