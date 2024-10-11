use crate::prelude::*;
use rand::Rng;

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct Bomb {
    pub full_duration: usize,
    pub time_until_explosion: f32,
    pub state: BombState,
    pub explosion_radius: f32,
}

impl Default for Bomb {
    fn default() -> Self {
        Self::new(BOMB_EXPLOSION_RADIUS)
    }
}

impl Bomb {
    pub fn new(explosion_radius: f32) -> Self {
        let mut rng = rand::thread_rng();
        let full_duration = if FunctionalityOverride::AllBombsExplodeAfterOneSecond.enabled() {
            1
        } else {
            rng.gen_range(BOMB_MIN_TIME..BOMB_MAX_TIME)
        };
        Bomb {
            full_duration,
            time_until_explosion: full_duration as f32,
            state: BombState::PreHeld,
            explosion_radius,
        }
    }
}
