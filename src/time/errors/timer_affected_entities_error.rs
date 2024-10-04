use std::fmt::{Debug, Display};

use crate::prelude::*;

#[derive(Debug, Copy, Clone)]
pub enum TimerAffectedEntitiesError {
    FoundNoAffectedEntityToMatchWith(TimerAffectedEntity),
    IndexOutOfRangeForAffectedEntities(usize),
    ItemWithAffectedEntityNotFound(Entity),
}

impl Display for TimerAffectedEntitiesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FoundNoAffectedEntityToMatchWith(affected_entity) => {
                write!(
                    f,
                    "Found no affected entity to match with {:?}",
                    affected_entity
                )
            }
            Self::IndexOutOfRangeForAffectedEntities(index) => write!(
                f,
                "Index {:?} out of range for timer affected entities",
                index
            ),
            Self::ItemWithAffectedEntityNotFound(entity) => {
                write!(f, "The timer doesn't affect this entity: {:?}", entity)
            }
        }
    }
}

impl From<VecBasedArrayError<TimerAffectedEntity, TimerAffectedEntity, TIMER_MAX_ASSIGNED_ENTITIES>>
    for TimerAffectedEntitiesError
{
    fn from(
        value: VecBasedArrayError<
            TimerAffectedEntity,
            TimerAffectedEntity,
            TIMER_MAX_ASSIGNED_ENTITIES,
        >,
    ) -> Self {
        match value {
            VecBasedArrayError::FoundNoItemToMatchWith(affected_entity, _) => {
                Self::FoundNoAffectedEntityToMatchWith(affected_entity)
            }
            VecBasedArrayError::IndexOutOfRange(index, _) => {
                Self::IndexOutOfRangeForAffectedEntities(index)
            }
            VecBasedArrayError::ItemWithAffectedEntityNotFound(entity) => {
                Self::ItemWithAffectedEntityNotFound(entity)
            }
        }
    }
}
