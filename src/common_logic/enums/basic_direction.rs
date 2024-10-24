use std::f32::consts::{PI, SQRT_2};

use crate::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Clone, Copy, EnumIter, Reflect)]
pub enum BasicDirection {
    DownLeft,
    Down,
    RightDown,
    Right,
    UpRight,
    Up,
    LeftUp,
    Left,
}

impl BasicDirection {
    pub fn closest_reversed(find_closest_reversed_to: Vec2) -> Self {
        let closest = Self::closest(find_closest_reversed_to);
        Self::index_to_dir((closest as u8 + 4) % 8).unwrap()
    }

    pub fn closest(find_closest_to: Vec2) -> Self {
        let angle = Vec2::X.angle_between(find_closest_to);
        let normalized_angle = (angle + PI) - (PI / 8.0);
        let positive_normalized = normalized_angle % (2.0 * PI);
        let angle_in_eight_turns = positive_normalized / (PI / 4.0);
        let rounded = angle_in_eight_turns.floor() as u8;
        Self::index_to_dir(rounded).unwrap()
    }

    pub fn to_initial_frame_index(&self) -> usize {
        match self {
            Self::DownLeft => 6,
            Self::Down => 7,
            Self::RightDown => 8,
            Self::Right => 5,
            Self::UpRight => 2,
            Self::Up => 1,
            Self::LeftUp => 0,
            Self::Left => 3,
        }
    }

    pub fn index_to_dir(index: u8) -> Option<Self> {
        for (direction_index, direction) in BasicDirection::iter().enumerate() {
            if direction_index == index as usize {
                return Some(direction);
            }
        }
        None
    }

    pub fn to_normalized_vec(&self) -> Vec2 {
        let inverted_sqrtwo = 1.0 / SQRT_2;
        match self {
            Self::DownLeft => Vec2::new(-inverted_sqrtwo, -inverted_sqrtwo),
            Self::Down => Vec2::new(0.0, -1.0),
            Self::RightDown => Vec2::new(inverted_sqrtwo, -inverted_sqrtwo),
            Self::Right => Vec2::new(1.0, 0.0),
            Self::UpRight => Vec2::new(inverted_sqrtwo, inverted_sqrtwo),
            Self::Up => Vec2::new(0.0, 1.0),
            Self::LeftUp => Vec2::new(-inverted_sqrtwo, inverted_sqrtwo),
            Self::Left => Vec2::new(-1.0, 0.0),
        }
    }

    pub fn diagonal(&self) -> bool {
        match self {
            Self::DownLeft | Self::RightDown | Self::UpRight | Self::LeftUp => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closest() {
        let almost_up_left = Vec2::new(-1.0, 0.9);
        let almost_down_left = Vec2::new(-0.85, -0.9);
        let almost_down_right = Vec2::new(3.2, -3.1);

        let should_be_up_left = BasicDirection::closest(almost_up_left);
        let should_be_down_left = BasicDirection::closest(almost_down_left);
        let should_be_down_right = BasicDirection::closest(almost_down_right);

        assert_eq!(BasicDirection::LeftUp, should_be_up_left);
        assert_eq!(BasicDirection::DownLeft, should_be_down_left);
        assert_eq!(BasicDirection::RightDown, should_be_down_right);
    }
}
