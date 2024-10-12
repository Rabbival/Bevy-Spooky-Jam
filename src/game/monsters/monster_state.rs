use bevy::color::palettes::{
    basic::{GRAY, MAROON, OLIVE},
    css::PINK,
};

use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub enum MonsterState {
    Spawning,
    #[default]
    Idle,
    Chasing(Entity),
    Fleeing(Vec3),
}

impl MonsterState {
    pub fn to_hearing_ring_gizmo_color(&self) -> Color {
        match self {
            MonsterState::Spawning => Color::from(PINK),
            MonsterState::Idle => Color::from(GRAY),
            MonsterState::Chasing(_) => Color::from(MAROON),
            MonsterState::Fleeing(_) => Color::from(OLIVE),
        }
    }
}
