use crate::prelude::*;
use std::time::Duration;

use bevy::time::Stopwatch;

#[derive(Component, Debug)]
pub struct WorldBoundsWrapped;

#[derive(Component, Debug)]
pub struct InWorldButNotBoundWrapped;

#[derive(Component, Default)]
pub struct PlayerGameStopwatchUi {
    pub timer: Stopwatch,
}

#[derive(Component, Default)]
pub struct PlayerScoreTextUi;

#[derive(Component, Default)]
pub struct LeaderboardScoreTextUi;

#[derive(Component, Default)]
pub struct WorldChampionshipLeaderboardScoring {
    pub elapsed: Duration,
    pub hi_score: u32,
}

#[derive(Component)]
pub struct AgainScreen;
