use crate::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct Orb;

#[derive(Debug, Clone, Copy, Component)]
pub struct Patroller;

#[derive(Component, Default)]
pub struct Monster {
    pub hearing_ring_distance: f32,
    pub state: MonsterState,
}

#[derive(Component, Eq, PartialEq, Default)]
pub enum MonsterState {
    #[default]
    Idle,
    Chasing,
    Fleeing,
    CalmingDown,
}

#[derive(Component)]
pub struct WorldBoundsWrapped;
