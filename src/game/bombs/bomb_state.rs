use bevy::color::{
    palettes::css::{DARK_RED, DARK_SLATE_GRAY, LIGHT_GRAY, YELLOW},
    Color,
};

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
                bomb: Color::from(DARK_SLATE_GRAY),
                text: Color::from(LIGHT_GRAY),
            }),
            Self::Held => Some(BombAndTextColors {
                bomb: Color::from(DARK_RED),
                text: Color::from(YELLOW),
            }),
            Self::PostHeld => None,
        }
    }
}
