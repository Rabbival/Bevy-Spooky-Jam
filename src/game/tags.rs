use crate::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct Orb;

#[derive(Debug, Clone, Copy, Component)]
pub struct Patroller;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Monster {
    pub hearing_ring_distance: f32,
    pub state: MonsterState,
}

#[derive(Component, Eq, PartialEq, Default)]
pub enum MonsterState {
    #[default]
    Idle,
    Alert
}

#[derive(Component)]
pub struct WorldBoundsWrapped;

#[derive(Component)]
pub struct Bomb;
