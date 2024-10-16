use std::fmt::{Debug, Display};

#[derive(Debug, Copy, Clone)]
pub enum TimerSequenceError {
    SequenceHasNoTimerInIndex(usize),
    TriedToFireATimerSequenceWithNoTimers,
    ATimerFromASequenceFinishedButParentNotFound,
}

impl Display for TimerSequenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::SequenceHasNoTimerInIndex(index) => {
                write!(
                    f,
                    "Tried to fire a sequence timer, but the sequence has no timer of index {:?}",
                    index
                )
            }
            Self::TriedToFireATimerSequenceWithNoTimers => {
                write!(f, "Tried to fire a timer sequence with an empty timer list")
            }
            Self::ATimerFromASequenceFinishedButParentNotFound => {
                write!(f, "A sequence timer finished but its sequence doesnt exist")
            }
        }
    }
}
