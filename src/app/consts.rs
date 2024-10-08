use bevy::math::vec2;
use crate::prelude::*;

pub const WINDOW_SIZE_IN_PIXELS: f32 = 900.0;
pub const SCREEN_COLOR_BACKGROUND: ClearColor = ClearColor(Color::srgb(0.1, 0.1, 0.1));
pub const TOP_UI_HEADER_BAR_SIZE: Vec2 = Vec2::new(WINDOW_SIZE_IN_PIXELS * 4.0 / 3.0, 60.0);
