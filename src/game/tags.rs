use crate::prelude::*;

use bevy::time::Stopwatch;

#[derive(Component, Debug)]
pub struct BombTag;

#[derive(Component, Debug, Default)]
pub struct BombAffected {
    pub currently_affected_by_bomb: bool,
}

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
pub struct BestScoreTextUi;

#[derive(Component, Default)]
pub struct LongestSurvivedUi;

#[derive(Component)]
pub struct AgainScreen;

#[derive(Component)]
pub struct DoNotDestroyOnRestart;
