use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct FacingDirection(pub Vec3);

impl FacingDirection {
    pub fn new(z_layer: f32) -> Self {
        Self(Vec2::Y.extend(z_layer))
    }
}
