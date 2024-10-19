use crate::prelude::*;

#[derive(Debug, Component)]
pub struct MusicLayer(pub u8);

#[derive(Debug, Component)]
pub struct AffectingTimeMultiplier(pub TimeMultiplierId);
