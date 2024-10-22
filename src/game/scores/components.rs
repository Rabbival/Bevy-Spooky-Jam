use crate::prelude::*;

#[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
pub struct CurrentGameScore(pub u32);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
pub struct BestScoreSoFar(pub u32);

#[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
pub struct LongestSurvivedSoFar(pub f32);
