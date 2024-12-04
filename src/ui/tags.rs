use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PlayerScoreTextUi;

#[derive(Component, Default)]
pub struct LastRunScoreTextUi;

#[derive(Component, Default)]
pub struct BestScoreTextUi;

#[derive(Component)]
pub enum InstructionsTextUi {
    Hold,
    Aim,
    Release,
}
