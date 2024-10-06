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
    pub last_player_location_seen: Vec3,
    pub bombs_location_seen: Vec<Vec3>,
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

#[derive(Component)]
pub struct Bomb;
