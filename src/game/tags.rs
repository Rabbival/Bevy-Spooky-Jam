use crate::prelude::*;
use bevy::color::palettes::css::*;

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

impl MonsterState {
    pub fn to_hearing_ring_gizmo_color(&self) -> Color {
        match self {
            MonsterState::Idle => Color::from(GRAY),
            MonsterState::Chasing => Color::from(MAROON),
            MonsterState::Fleeing => Color::from(OLIVE),
            MonsterState::CalmingDown => Color::from(NAVY),
        }
    }
}

#[derive(Component)]
pub struct WorldBoundsWrapped;
