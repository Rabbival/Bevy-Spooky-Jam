use bevy::color::palettes::basic::{GRAY, MAROON, NAVY, OLIVE};

use crate::prelude::{Color, MonsterState};

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
