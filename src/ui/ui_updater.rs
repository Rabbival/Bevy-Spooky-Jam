use crate::prelude::*;

pub struct UiUpdaterPlugin;

impl Plugin for UiUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_player_scoring,
                update_high_score,
                update_last_game_score_text,
            ),
        );
    }
}

fn update_player_scoring(
    changed_game_score_query: Query<&CurrentGameScore, Changed<CurrentGameScore>>,
    mut player_scoring_text_query: Query<&mut Text, With<PlayerScoreTextUi>>,
) {
    for game_score in &changed_game_score_query {
        for mut player_scoring_text in &mut player_scoring_text_query {
            player_scoring_text.sections[0].value =
                format!("Score: {:0>7}", game_score.0.to_string());
        }
    }
}

fn update_high_score(
    changed_best_score_query: Query<&BestScoreSoFar, Changed<BestScoreSoFar>>,
    mut high_score_text_query: Query<&mut Text, With<BestScoreTextUi>>,
) {
    for best_score in &changed_best_score_query {
        for mut high_score_text in &mut high_score_text_query {
            high_score_text.sections[0].value =
                format!("Best Score: {:0>7}", best_score.0.to_string());
        }
    }
}

fn update_last_game_score_text(
    last_game_score_query: Query<&LastGameScore, Changed<LastGameScore>>,
    mut last_run_score_text_query: Query<&mut Text, With<LastRunScoreTextUi>>,
) {
    for last_game_score in &last_game_score_query {
        for mut last_run_score_text in &mut last_run_score_text_query {
            last_run_score_text.sections[0].value =
                format!("Score: {:0>7}", last_game_score.0.to_string());
        }
    }
}
