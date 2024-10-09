use bevy::color::{
    palettes::css::{ANTIQUE_WHITE, DARK_RED, DARK_SLATE_GRAY, YELLOW},
    Color, Srgba,
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
                bomb: Color::from(ANTIQUE_WHITE),
                text: Color::from(Srgba::new(0.0, 0.0, 0.0, 1.0)),
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
