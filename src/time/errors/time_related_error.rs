use std::fmt::{Debug, Display};

use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum TimeRelatedError {
    TimeMultiplierNotFound(TimeMultiplierId),
    AttemptedToChangeFixedTimeMultiplier(TimeMultiplierId),
    TimerToRemoveFromNotFound(RemoveFromTimerAffectedEntities),
    TimerAffectedEntitiesError(TimerAffectedEntitiesError),
}

impl Display for TimeRelatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::TimeMultiplierNotFound(id) => {
                write!(f, "Time processor with id {:?} not found", id)
            }
            Self::AttemptedToChangeFixedTimeMultiplier(id) => write!(
                f,
                "Attempted to change fixed multiplier time processor with id {:?}",
                id
            ),
            Self::TimerToRemoveFromNotFound(event) => {
                write!(
                    f,
                    "Couldn't find timer to remove entity from. Event: {:?}",
                    event
                )
            }
            Self::TimerAffectedEntitiesError(timer_affected_entity_error) => {
                write!(
                    f,
                    "Error when accessing affected entities: {}",
                    timer_affected_entity_error
                )
            }
        }
    }
}

impl From<VecBasedArrayError<TimerAffectedEntity, TimerAffectedEntity, TIMER_MAX_ASSIGNED_ENTITIES>>
    for TimeRelatedError
{
    fn from(
        value: VecBasedArrayError<
            TimerAffectedEntity,
            TimerAffectedEntity,
            TIMER_MAX_ASSIGNED_ENTITIES,
        >,
    ) -> Self {
        Self::TimerAffectedEntitiesError(TimerAffectedEntitiesError::from(value))
    }
}
