use std::fmt::{Debug, Display};

#[derive(Debug, Copy, Clone)]
pub enum MonsterError {
    CouldntFindAPlaceToSpawnMonsterIn,
}

impl Display for MonsterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CouldntFindAPlaceToSpawnMonsterIn => {
                write!(
                    f,
                    "A monster spawn was requested but no valid place for it was found"
                )
            }
        }
    }
}
