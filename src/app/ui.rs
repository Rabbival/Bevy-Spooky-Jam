use crate::prelude::*;

use bevy::text::Text2dBounds;
use bevy_light_2d::prelude::{LightOccluder2d, LightOccluder2dShape};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);

        if FunctionalityOverride::DontUpdateUI.disabled() {
            app.add_systems(
                Update,
                (
                    update_player_scoring,
                    update_high_score,
                    update_last_game_score_text,
                ),
            );
        }
    }
}

fn spawn_ui(
    image_fonts_resource: ResMut<StaticImageHandles>,
    text_fonts_resource: ResMut<TextFonts>,
    mut commands: Commands,
) {
    commands.spawn((
        SpriteBundle {
            texture: image_fonts_resource.floor_image_handle.clone(),
            ..default()
        },
        DoNotDestroyOnRestart,
    ));
    let text_color = Color::srgba(0.9, 0.9, 0.9, 1.0);
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.3, 0.3, 0.3, 1.0),
                custom_size: Some(Vec2::new(WINDOW_SIZE_IN_PIXELS, TOP_UI_HEADER_BAR_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(
                Vec2::new(
                    0.0,
                    (WINDOW_SIZE_IN_PIXELS / 2.0) + (TOP_UI_HEADER_BAR_HEIGHT / 2.0),
                )
                .extend(100.0),
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
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Score: 0000000",
                TextStyle {
                    font: text_fonts_resource.kenny_high_square_handle.clone(),
                    font_size: 40.0,
                    color: text_color,
                },
            )
            .with_justify(JustifyText::Left),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(
                    WINDOW_SIZE_IN_PIXELS / 2.0,
                    (WINDOW_SIZE_IN_PIXELS / 2.0) + (TOP_UI_HEADER_BAR_HEIGHT / 2.0),
                ),
            },
            transform: Transform::from_translation(
                Vec2::new(
                    -WINDOW_SIZE_IN_PIXELS / 3.0,
                    (WINDOW_SIZE_IN_PIXELS / 2.0) + (TOP_UI_HEADER_BAR_HEIGHT / 2.0),
                )
                .extend(101.0),
            ),
            ..default()
        },
        PlayerScoreTextUi,
        DoNotDestroyOnRestart,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Best Score: 0000000",
                TextStyle {
                    font: text_fonts_resource.kenny_high_square_handle.clone(),
                    font_size: 40.0,
                    color: text_color,
                },
            )
            .with_justify(JustifyText::Left),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(
                    WINDOW_SIZE_IN_PIXELS / 2.0,
                    (WINDOW_SIZE_IN_PIXELS / 2.0) + (TOP_UI_HEADER_BAR_HEIGHT / 2.0),
                ),
            },
            transform: Transform::from_translation(
                Vec2::new(
                    11.0 * WINDOW_SIZE_IN_PIXELS / 36.0,
                    (WINDOW_SIZE_IN_PIXELS / 2.0) + (TOP_UI_HEADER_BAR_HEIGHT / 2.0),
                )
                .extend(101.0),
            ),
            ..default()
        },
        BestScoreTextUi,
        DoNotDestroyOnRestart,
    ));
}

fn update_player_scoring(
    changed_game_score_query: Query<&CurrentGameScore, Changed<CurrentGameScore>>,
    mut player_scoring_text_query: Query<&mut Text, With<PlayerScoreTextUi>>,
) {
    for game_score in &changed_game_score_query {
        for mut player_scoring_text in &mut player_scoring_text_query {
            player_scoring_text.sections[0].value =
                format!("Score: {:0>7}", game_score.0.to_string());
        }
    }
}

fn update_high_score(
    changed_best_score_query: Query<&BestScoreSoFar, Changed<BestScoreSoFar>>,
    mut high_score_text_query: Query<&mut Text, With<BestScoreTextUi>>,
) {
    for best_score in &changed_best_score_query {
        for mut high_score_text in &mut high_score_text_query {
            high_score_text.sections[0].value =
                format!("Best Score: {:0>7}", best_score.0.to_string());
        }
    }
}

fn update_last_game_score_text(
    last_game_score_query: Query<&LastGameScore, Changed<LastGameScore>>,
    mut last_run_score_text_query: Query<&mut Text, With<LastRunScoreTextUi>>,
) {
    for last_game_score in &last_game_score_query {
        for mut last_run_score_text in &mut last_run_score_text_query {
            last_run_score_text.sections[0].value =
                format!("Score: {:0>7}", last_game_score.0.to_string());
        }
    }
}
