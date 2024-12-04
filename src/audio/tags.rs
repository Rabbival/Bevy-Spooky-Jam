use crate::prelude::*;

#[derive(Debug, Component)]
pub enum MusicLayer {
    Base,
    Chase,
}

#[derive(Debug, Component)]
pub struct AffectingTimeMultiplier(pub TimeMultiplierId);
