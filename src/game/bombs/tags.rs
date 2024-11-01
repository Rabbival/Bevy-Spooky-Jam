use bevy::math::bounding::BoundingCircle;

use crate::prelude::*;

#[derive(Component, Debug)]
pub struct BombTag;

#[derive(Component, Debug, Default)]
pub struct BombAffected {
    pub currently_affected_by_bomb: bool,
}

#[derive(Component, Debug)]
pub struct BombExplosionPreview;

#[derive(Component, Debug)]
pub struct ExplodeInContact {
    pub bounding_circle: BoundingCircle,
}
