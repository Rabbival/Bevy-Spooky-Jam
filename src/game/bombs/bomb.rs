use crate::prelude::*;
use rand::Rng;

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct Bomb {
    pub full_duration: usize,
    pub currently_displayed: usize,
    pub bomb_state: BombState,
}

impl Bomb {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let full_duration = rng.gen_range(BOMB_MIN_TIME..BOMB_MAX_TIME);
        Bomb {
            full_duration,
            currently_displayed: full_duration,
            bomb_state: BombState::PreHeld,
        }
    }
}
