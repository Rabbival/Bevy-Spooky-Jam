use crate::prelude::*;

#[derive(Component, Debug)]
pub struct WorldBoundsWrapped;

#[derive(Component, Debug)]
pub struct InWorldButNotBoundWrapped;

#[derive(Component, Default)]
pub struct PlayerScoreTextUi;

#[derive(Component, Default)]
pub struct LastRunScoreTextUi;

#[derive(Component, Default)]
pub struct BestScoreTextUi;

#[derive(Component)]
pub struct AgainScreen;

#[derive(Component)]
pub struct DoNotDestroyOnRestart;
