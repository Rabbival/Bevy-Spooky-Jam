use bevy::text::Text2dBounds;
use bevy_light_2d::prelude::{LightOccluder2d, LightOccluder2dShape};

use crate::prelude::*;

pub struct UiSpawnerPlugin;

impl Plugin for UiSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);
    }
}

fn spawn_ui(text_fonts_resource: Res<TextFonts>, mut commands: Commands) {
    let text_color = Color::srgba(0.9, 0.9, 0.9, 1.0);
    spawn_score_bar(&mut commands);
    spawn_score_text(&text_fonts_resource, text_color, &mut commands);
    spawn_best_score_text(&text_fonts_resource, text_color, &mut commands);
}

fn spawn_score_bar(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.3, 0.3, 0.3, 1.0),
                custom_size: Some(Vec2::new(WINDOW_SIZE_IN_PIXELS, TOP_UI_HEADER_BAR_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(
                Vec2::new(0.0, SCORE_BAR_HEIGHT).extend(SCORE_BAR_LAYER),
            ),
            ..default()
        },
        LightOccluder2d {
            shape: LightOccluder2dShape::Rectangle {
                half_size: Vec2::new(WINDOW_SIZE_IN_PIXELS / 2.0, TOP_UI_HEADER_BAR_HEIGHT / 2.0),
            },
        },
        DoNotDestroyOnRestart,
    ));
}

fn spawn_score_text(text_fonts: &TextFonts, text_color: Color, commands: &mut Commands) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Score: 0000000",
                TextStyle {
                    font: text_fonts.kenny_high_square_handle.clone(),
                    font_size: 40.0,
                    color: text_color,
                },
            ),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(WINDOW_SIZE_IN_PIXELS, WINDOW_SIZE_IN_PIXELS / 2.0),
            },
            transform: Transform::from_translation(
                Vec2::new(-WINDOW_SIZE_IN_PIXELS / 3.0, SCORE_BAR_HEIGHT).extend(SCORE_TEXT_LAYER),
            ),
            ..default()
        },
        PlayerScoreTextUi,
        DoNotDestroyOnRestart,
    ));
}

fn spawn_best_score_text(text_fonts: &TextFonts, text_color: Color, commands: &mut Commands) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Best Score: 0000000",
                TextStyle {
                    font: text_fonts.kenny_high_square_handle.clone(),
                    font_size: 40.0,
                    color: text_color,
                },
            ),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(WINDOW_SIZE_IN_PIXELS, WINDOW_SIZE_IN_PIXELS / 2.0),
            },
            transform: Transform::from_translation(
                Vec2::new(14.0 * WINDOW_SIZE_IN_PIXELS / 47.0, SCORE_BAR_HEIGHT)
                    .extend(SCORE_TEXT_LAYER),
            ),
            ..default()
        },
        BestScoreTextUi,
        DoNotDestroyOnRestart,
    ));
}
