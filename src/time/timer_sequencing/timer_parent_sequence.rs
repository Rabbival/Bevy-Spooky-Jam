use bevy::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct TimerParentSequence {
    pub parent_sequence: Entity,
    pub index_in_sequence: usize,
}
