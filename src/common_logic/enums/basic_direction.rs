use crate::prelude::*;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Copy, EnumIter)]
pub enum BasicDirection {
    Up,
    Right,
    Down,
    Left,
}

impl BasicDirection {
    pub fn opposite_direction_index(&self) -> u8 {
        let index = *self as u8;
        (index + 2) % 4
    }

    pub fn opposite_direction(&self) -> Option<Self> {
        Self::index_to_dir(self.opposite_direction_index())
    }

    pub fn to_world_direction(&self) -> Vec2 {
        match self {
            Self::Up => Vec2::Y,
            Self::Right => Vec2::X,
            Self::Down => Vec2::NEG_Y,
            Self::Left => Vec2::NEG_X,
        }
    }

    pub fn to_rotation(&self) -> Quat {
        Quat::from_rotation_arc_2d(Vec2::Y, self.to_world_direction())
    }
}

impl BasicDirection {
    pub fn index_to_dir(index: u8) -> Option<Self> {
        match index {
            0 => Some(BasicDirection::Up),
            1 => Some(BasicDirection::Right),
            2 => Some(BasicDirection::Down),
            3 => Some(BasicDirection::Left),
            _ => None,
        }
    }

    pub fn from_keycode(keycode: &KeyCode) -> Option<BasicDirection> {
        match keycode {
            KeyCode::KeyW | KeyCode::ArrowUp => Some(BasicDirection::Up),
            KeyCode::KeyD | KeyCode::ArrowRight => Some(BasicDirection::Right),
            KeyCode::KeyS | KeyCode::ArrowDown => Some(BasicDirection::Down),
            KeyCode::KeyA | KeyCode::ArrowLeft => Some(BasicDirection::Left),
            _ => None,
        }
    }
}
