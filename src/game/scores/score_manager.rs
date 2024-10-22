use crate::prelude::*;
use crate::read_no_field_variant;
use crate::single_mut_else_return;

pub struct ScoreManagerPlugin;

impl Plugin for ScoreManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_score_entities)
            .add_systems(Update, (update_player_scoring, listen_for_game_over_events));
    }
}

fn spawn_score_entities(mut commands: Commands) {
    commands.spawn((CurrentGameScore::default(), DoNotDestroyOnRestart));
    commands.spawn((BestScoreSoFar::default(), DoNotDestroyOnRestart));
    commands.spawn((LongestSurvivedSoFar::default(), DoNotDestroyOnRestart));
}

fn update_player_scoring(
    mut game_score_query: Query<&mut CurrentGameScore>,
    mut events_reader: EventReader<AppendToPlayerScoreEvent>,
) {
    for event in events_reader.read() {
        let mut game_score = single_mut_else_return!(game_score_query);
        game_score.0 += event.0;
    }
}

fn listen_for_game_over_events(
    mut game_events_listener: EventReader<GameEvent>,
    mut game_score_query: Query<&mut CurrentGameScore>,
    mut best_score_query: Query<&mut BestScoreSoFar>,
    mut longest_survived_query: Query<&mut LongestSurvivedSoFar>,
    mut stopwatch_query: Query<&mut PlayerGameStopwatchUi>,
) {
    if read_no_field_variant!(game_events_listener, GameEvent::GameOver).count() > 0 {
        let mut game_score = single_mut_else_return!(game_score_query);
        let mut best_score = single_mut_else_return!(best_score_query);
        let mut longest_survived = single_mut_else_return!(longest_survived_query);
        let mut stopwatch = single_mut_else_return!(stopwatch_query);
        if game_score.0 > best_score.0 {
            best_score.0 = game_score.0;
        }
        game_score.0 = 0;
        if stopwatch.timer.elapsed_secs() > longest_survived.0 {
            longest_survived.0 = stopwatch.timer.elapsed_secs();
        }
        stopwatch.timer.reset();
    }
}
