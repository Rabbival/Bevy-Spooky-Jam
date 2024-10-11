use crate::prelude::*;

use bevy::text::Text2dBounds;
use bevy::time::Stopwatch;

use super::assets_loader::TextFonts;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);

        if FunctionalityOverride::DontUpdateUI.disabled() {
            app.add_systems(
                Update,
                (update_player_game_stopwatch, update_player_scoring),
            );
        }
    }
}

fn spawn_ui(text_fonts_resource: ResMut<TextFonts>, mut commands: Commands) {
    let text_color = Color::srgba(0.9, 0.9, 0.9, 1.0);
    commands.spawn(SpriteBundle {
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
    });
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "00'00''00",
                TextStyle {
                    font: text_fonts_resource.kenny_blocks_handle.clone(),
                    font_size: 60.0,
                    color: text_color,
                },
            )
            .with_justify(JustifyText::Left),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(WINDOW_SIZE_IN_PIXELS, TOP_UI_HEADER_BAR_HEIGHT),
            },
            transform: Transform::from_translation(
                Vec2::new(
                    0.0,
                    (WINDOW_SIZE_IN_PIXELS / 2.0) + (TOP_UI_HEADER_BAR_HEIGHT / 2.0),
                )
                .extend(101.0),
            ),
            ..default()
        },
        PlayerGameStopwatchUi { ..default() },
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Score: 1.000.000",
                TextStyle {
                    font: text_fonts_resource.kenny_high_square_handle.clone(),
                    font_size: 30.0,
                    color: text_color,
                    ..default()
                },
            )
            .with_justify(JustifyText::Left),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(
                    WINDOW_SIZE_IN_PIXELS / 3.0,
                    (WINDOW_SIZE_IN_PIXELS / 2.0) + (TOP_UI_HEADER_BAR_HEIGHT / 2.0),
                ),
            },
            transform: Transform::from_translation(
                Vec2::new(
                    (-WINDOW_SIZE_IN_PIXELS / 2.0) + 100.0,
                    (WINDOW_SIZE_IN_PIXELS / 2.0) + (TOP_UI_HEADER_BAR_HEIGHT / 2.0),
                )
                .extend(101.0),
            ),
            ..default()
        },
        PlayerScoreTextUi,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Hi  Score: 1000000",
                TextStyle {
                    font: text_fonts_resource.kenny_high_square_handle.clone(),
                    font_size: 30.0,
                    color: text_color,
                },
            )
            .with_justify(JustifyText::Left),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(
                    WINDOW_SIZE_IN_PIXELS / 3.0,
                    (WINDOW_SIZE_IN_PIXELS / 2.0) + (TOP_UI_HEADER_BAR_HEIGHT / 2.0),
                ),
            },
            transform: Transform::from_translation(
                Vec2::new(
                    (WINDOW_SIZE_IN_PIXELS / 2.0) - 110.0,
                    (WINDOW_SIZE_IN_PIXELS / 2.0) + (TOP_UI_HEADER_BAR_HEIGHT / 2.0),
                )
                .extend(101.0),
            ),
            ..default()
        },
        LeaderboardScoreTextUi,
    ));
}

fn update_player_game_stopwatch(
    mut player_game_stopwatch_text_query: Query<
        (&mut Text, &mut PlayerGameStopwatchUi),
        With<PlayerGameStopwatchUi>,
    >,
    time: Res<Time>,
) {
    for (mut text, mut stopwatch) in player_game_stopwatch_text_query.iter_mut() {
        stopwatch.timer.tick(time.delta());
        text.sections[0].value = get_elapsed_secs_as_a_formatted_string(stopwatch.timer.clone());
    }
}

fn update_player_scoring(
    players_query: Query<&Player>,
    mut player_scoring_text_query: Query<&mut Text, With<PlayerScoreTextUi>>,
) {
    for player in players_query.iter() {
        for mut player_scoring_text in player_scoring_text_query.iter_mut() {
            player_scoring_text.sections[0].value =
                format!("Score: {:0>7}", player.score.to_string());
        }
    }
}

fn get_elapsed_secs_as_a_formatted_string(timer: Stopwatch) -> String {
    let minutes = (timer.elapsed_secs() / 60.0) as i32;
    let seconds = (timer.elapsed_secs() % 60.0) as i32;
    let milliseconds = (timer.elapsed_secs().fract() * 100.0) as i32;
    format!(
        "{:0>2}'{:0>2}''{:0>2}",
        minutes.to_string(),
        seconds.to_string(),
        milliseconds.to_string()
    )
}
