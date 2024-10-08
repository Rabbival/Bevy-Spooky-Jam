use crate::prelude::*;

use bevy::time::Stopwatch;

#[derive(Component)]
pub struct WorldBoundsWrapped;

#[derive(Component, Default)]
pub struct PlayerGameStopwatch {
    pub timer: Stopwatch,
}
