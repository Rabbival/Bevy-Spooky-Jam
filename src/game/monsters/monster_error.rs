use std::fmt::{Debug, Display};

use super::{MonsterState, TimerSequenceError};

#[derive(Debug, Copy, Clone)]
pub enum MonsterError {
    CouldntFindAPlaceToSpawnMonsterIn,
    TimerSequenceError(TimerSequenceError),
    NoPathSequenceFoundOnStateChange(MonsterState),
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
            Self::TimerSequenceError(timer_sequence_error) => {
                write!(f, "{}", timer_sequence_error)
            }
            Self::NoPathSequenceFoundOnStateChange(next_state) => {
                write!(
                    f,
                    "Tried to manipulate path when changing to state {:?} but no path dequence was found",
                    next_state
                )
            }
        }
    }
}

impl From<TimerSequenceError> for MonsterError {
    fn from(value: TimerSequenceError) -> Self {
        Self::TimerSequenceError(value)
    }
}
