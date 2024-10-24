use crate::prelude::*;

#[derive(Component, Debug)]
pub struct BoundsWrappedPhantom {
    pub index: usize,
}

impl BoundsWrappedPhantom {
    pub fn relative_location_to_parent(
        parent_location: Vec2,
    ) -> [Vec2; BOUNDS_WRAPPED_PHANTOMS_PER_PARENT] {
        Self::screens_from_parent_location(parent_location).map(|screen| {
            screen.to_normalized_vec()
                * if screen.diagonal() {
                    (WINDOW_SIZE_IN_PIXELS.powf(2.0) * 2.0).sqrt()
                } else {
                    WINDOW_SIZE_IN_PIXELS
                }
        })
    }

    fn screens_from_parent_location(
        parent_location: Vec2,
    ) -> [BasicDirection; BOUNDS_WRAPPED_PHANTOMS_PER_PARENT] {
        [
            BasicDirection::closest_reversed(Vec2::new(parent_location.x, 0.0)),
            BasicDirection::closest_reversed(parent_location),
            BasicDirection::closest_reversed(Vec2::new(0.0, parent_location.y)),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_screens_from_parent_location() {
        let almost_up_left = Vec2::new(-1.0, 0.9);
        let almost_down_left = Vec2::new(-0.85, -0.9);
        let almost_down_right = Vec2::new(3.2, -3.1);

        let from_almost_up_left =
            BoundsWrappedPhantom::screens_from_parent_location(almost_up_left);
        let from_almost_down_left =
            BoundsWrappedPhantom::screens_from_parent_location(almost_down_left);
        let from_almost_down_right =
            BoundsWrappedPhantom::screens_from_parent_location(almost_down_right);

        assert_eq!(
            from_almost_up_left,
            [
                BasicDirection::Right,
                BasicDirection::RightDown,
                BasicDirection::Down,
            ]
        );
        assert_eq!(
            from_almost_down_left,
            [
                BasicDirection::Right,
                BasicDirection::UpRight,
                BasicDirection::Up,
            ]
        );
        assert_eq!(
            from_almost_down_right,
            [
                BasicDirection::Left,
                BasicDirection::LeftUp,
                BasicDirection::Up,
            ]
        );
    }
}
