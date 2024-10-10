use bevy::color::{
    palettes::css::{ANTIQUE_WHITE, DARK_RED, DARK_SLATE_GRAY},
    Color, Srgba,
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
                bomb: Color::from(DARK_RED),
                text: Color::from(DARK_SLATE_GRAY),
            }),
            Self::Held => Some(BombAndTextColors {
                bomb: Color::from(ANTIQUE_WHITE),
                text: Color::from(Srgba::new(0.0, 0.0, 0.0, 1.0)),
            }),
            Self::PostHeld => None,
        }
    }
}
