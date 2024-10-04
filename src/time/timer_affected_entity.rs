use crate::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TimerAffectedEntity {
    pub affected_entity: Entity,
    pub value_calculator_entity: Option<Entity>,
}
