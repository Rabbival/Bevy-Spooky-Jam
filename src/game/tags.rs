use crate::prelude::*;

#[derive(Component)]
pub struct WorldBoundsWrapped;

#[derive(Component)]
pub struct PlayerGameStopwatch {
    pub elapsed_ms: u64,
}
