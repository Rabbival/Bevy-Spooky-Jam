use crate::prelude::*;

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Monster {
    pub hearing_ring_distance: f32,
    pub state: MonsterState,
    pub path_timer_sequence: Entity,
}
