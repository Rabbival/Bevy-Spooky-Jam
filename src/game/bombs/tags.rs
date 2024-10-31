use crate::prelude::*;

#[derive(Component, Debug)]
pub struct BombTag;

#[derive(Component, Debug, Default)]
pub struct BombAffected {
    pub currently_affected_by_bomb: bool,
}

#[derive(Component, Debug)]
pub struct BombPreviewedExplosionLocation;
