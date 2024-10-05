use std::fmt::{Debug, Display};

#[derive(Debug, Copy, Clone)]
pub enum BombError {
    CouldntFindAPlaceToSpawnBombIn,
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
        }
    }
}
