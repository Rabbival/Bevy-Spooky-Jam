use strum_macros::EnumIter;

use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash, Default, EnumIter)]
pub enum TimeMultiplierId {
    #[default]
    RealTime,
    GameTimeMultiplier,
    UiTimeMultiplier,
}

impl TimeMultiplierId {
    pub fn to_initial_properties(&self) -> TimeMultiplier {
        TimeMultiplier::new(
            *self,
            DEFAULT_TIME_MULTIPLIER,
            *self != TimeMultiplierId::default(),
        )
    }
}
