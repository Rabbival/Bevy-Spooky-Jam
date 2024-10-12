use crate::prelude::*;
use crate::single_mut_else_return;

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
    mut events_reader: EventReader<AppendToPlayerScoreEvent>,
) {
    for event in events_reader.read() {
        let mut world_championship_leaderboard_scoring =
            single_mut_else_return!(world_championship_leaderboard_scoring_query);
        for mut player in &mut players_query {
            player.score += event.0;
            if world_championship_leaderboard_scoring.hi_score < player.score {
                world_championship_leaderboard_scoring.hi_score = player.score;
            }
        }
    }
}
