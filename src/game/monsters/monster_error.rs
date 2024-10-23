use std::fmt::{Debug, Display};

use super::{MonsterState, TimerSequenceError};

#[derive(Debug, Copy, Clone)]
pub enum MonsterError {
    CouldntFindAPlaceToSpawnMonsterIn,
    TimerSequenceError(TimerSequenceError),
    NoPathSequenceFoundOnStateChange(MonsterState),
    NoMovementAffectingTimerFound,
    NoMovementTimerHadTheListedPathParentSequence,
    MonsterHasNoPathTimerSequenceAssigned,
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
                    "Tried to manipulate path when changing to state {:?} but no path sequence was found",
                    next_state
                )
            }
            Self::NoMovementAffectingTimerFound => {
                write!(
                    f,
                    "Monster has no movement timer listed in its affecting timers"
                )
            }
            Self::NoMovementTimerHadTheListedPathParentSequence => {
                write!(
                    f,
                    "Found movement affecting timers, but none has the sequence listed in the monster's struct"
                )
            }
            Self::MonsterHasNoPathTimerSequenceAssigned => {
                write!(f, "Monster path sequence entity is None")
            }
        }
    }
}

impl From<TimerSequenceError> for MonsterError {
    fn from(value: TimerSequenceError) -> Self {
        Self::TimerSequenceError(value)
    }
}
