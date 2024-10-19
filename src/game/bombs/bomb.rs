use crate::prelude::*;
use bevy::color::palettes::css::{DARK_ORANGE, RED, SILVER, WHITE, YELLOW};
use rand::Rng;

#[derive(Component, Debug, PartialEq, Clone, Copy)]
pub struct Bomb {
    pub full_duration: usize,
    pub time_until_explosion: f32,
    pub state: BombState,
    pub explosion_radius: f32,
    pub about_to_explode: bool,
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
            about_to_explode: full_duration <= ABOUT_TO_EXPLODE_TIME_CIEL,
        }
    }

    pub fn to_colors(&self) -> Option<BombAndTextColors> {
        if self.about_to_explode {
            Some(BombAndTextColors {
                bomb: Color::from(RED),
                text: Color::from(WHITE),
            })
        } else {
            match self.state {
                BombState::PreHeld => Some(BombAndTextColors {
                    bomb: Color::from(WHITE),
                    text: Color::from(SILVER),
                }),
                BombState::Held => Some(BombAndTextColors {
                    bomb: Color::from(DARK_ORANGE),
                    text: Color::from(YELLOW),
                }),
                _ => None,
            }
        }
    }
}

pub struct BombAndTextColors {
    pub bomb: Color,
    pub text: Color,
}
