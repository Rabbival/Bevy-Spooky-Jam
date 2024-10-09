use bevy::color::{
    palettes::css::{DARK_RED, DARK_SLATE_GRAY, YELLOW},
    Color,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BombState {
    PreHeld,
    Held,
    Ticking,
}

pub struct BombAndTextColors {
    pub bomb: Color,
    pub text: Color,
}

impl BombState {
    pub fn to_color(&self) -> BombAndTextColors {
        match self {
            Self::PreHeld => BombAndTextColors {
                bomb: Color::from(DARK_SLATE_GRAY),
                text: Color::from(DARK_SLATE_GRAY),
            },
            Self::Held => BombAndTextColors {
                bomb: Color::from(DARK_RED),
                text: Color::from(DARK_SLATE_GRAY),
            },
            Self::Ticking => BombAndTextColors {
                bomb: Color::from(DARK_RED),
                text: Color::from(YELLOW),
            },
        }
    }
}
