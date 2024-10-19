use std::fmt::{Debug, Display};

#[derive(Debug, Copy, Clone)]
pub enum BombError {
    CouldntFindAPlaceToSpawnBombIn,
    AskedToTickAPreHeldBomb,
    CouldntParseTimerIntoInteger,
}

impl Display for BombError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CouldntFindAPlaceToSpawnBombIn => {
                write!(
                    f,
                    "A bomb spawn was requested but no valid place for it was found"
                )
            }
            Self::AskedToTickAPreHeldBomb => {
                write!(
                    f,
                    "A bomb ticking request was sent but the bomb wasn't yet held",
                )
            }
            Self::CouldntParseTimerIntoInteger => {
                write!(
                    f,
                    "Wanted to update a bombs text but it couldn't be parsed into an integer"
                )
            }
        }
    }
}
