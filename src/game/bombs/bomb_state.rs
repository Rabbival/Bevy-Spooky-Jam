use bevy::color::palettes::css::{DARK_ORANGE, SILVER, WHITE};
use bevy::color::Color;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BombState {
    PreHeld,
    Held,
    PostHeld,
}

pub struct BombAndTextColors {
    pub bomb: Color,
    pub text: Color,
}

impl BombState {
    pub fn to_colors(&self) -> Option<BombAndTextColors> {
        match self {
            Self::PreHeld => Some(BombAndTextColors {
                bomb: Color::from(WHITE),
                text: Color::from(SILVER),
            }),
            Self::Held => Some(BombAndTextColors {
                bomb: Color::from(DARK_ORANGE),
                text: Color::from(SILVER),
            }),
            Self::PostHeld => None,
        }
    }
}
