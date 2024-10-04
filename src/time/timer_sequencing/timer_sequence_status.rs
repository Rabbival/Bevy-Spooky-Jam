use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct TimerSequenceStatus {
    pub next_timer_index: Option<usize>,
    pub sequence_done: bool,
}
