use std::f32::consts::PI;

use crate::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Copy, EnumIter, Reflect)]
pub enum BasicDirection {
    Up,
    UpRight,
    Right,
    RightDown,
    Down,
    DownLeft,
    Left,
    LeftUp,
}

impl BasicDirection {
    // pub fn closest(find_closest_to: Vec2) -> BasicDirection {
    //     let angle = find_closest_to.angle_between(Vec2::X);

    // }

    pub fn opposite_direction_index(&self) -> u8 {
        let index = *self as u8;
        (index + 4) % 8
    }

    pub fn opposite_direction(&self) -> Option<Self> {
        Self::index_to_dir(self.opposite_direction_index())
    }

    pub fn index_to_dir(index: u8) -> Option<Self> {
        for (direction_index, direction) in BasicDirection::iter().enumerate() {
            if direction_index == index as usize {
                return Some(direction);
            }
        }
        None
    }
}
