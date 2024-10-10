use crate::prelude::ACTIVE_FUNCTIONALITY_OVERRIDES;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionalityOverride {
    AlwaysSpawnBombsInMiddle,
    PlayerMayCarryInfiniteBombs,
    AllBombsExplodeAfterOneSecond,
    EnemiesDontMove,
    DontSpawnUI,
}

impl FunctionalityOverride {
    pub fn enabled(&self) -> bool {
        ACTIVE_FUNCTIONALITY_OVERRIDES.contains(self)
    }

    pub fn disabled(&self) -> bool {
        !self.enabled()
    }
}
