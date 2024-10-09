use std::time::Duration;
use crate::prelude::*;

use bevy::time::Stopwatch;

#[derive(Component)]
pub struct WorldBoundsWrapped;

#[derive(Component, Default)]
pub struct PlayerGameStopwatchUi {
    pub timer: Stopwatch,
}

#[derive(Component, Default)]
pub struct PlayerScoreTextUi;

#[derive(Component, Default)]
pub struct WorldChampionshipLeaderboardScoring {
    pub elapsed: Duration,
    pub hi_score: u32,
}
