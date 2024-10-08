use crate::prelude::*;

#[derive(Component)]
pub struct WorldBoundsWrapped;

#[derive(Component)]
pub struct PlayerGameStopwatch {
    elapsed_ms: u64,
}
