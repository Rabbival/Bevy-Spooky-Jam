use crate::prelude::*;

use bevy::text::Text2dBounds;
use bevy::time::Stopwatch;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui)
            .add_systems(Update, update_player_game_stopwatch);
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.55),
            custom_size: Some(TOP_UI_HEADER_BAR_SIZE),
            ..default()
        },
        transform: Transform::from_translation(
            Vec2::new(
                0.0,
                (WINDOW_SIZE_IN_PIXELS / 2.0) - (TOP_UI_HEADER_BAR_SIZE.y / 2.0),
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
                    font_size: 60.0,
                    color: Color::BLACK,
                    ..default()
                },
            )
            .with_justify(JustifyText::Left),
            text_2d_bounds: Text2dBounds {
                size: TOP_UI_HEADER_BAR_SIZE,
            },
            transform: Transform::from_translation(
                Vec2::new(
                    0.0,
                    (WINDOW_SIZE_IN_PIXELS / 2.0) - (TOP_UI_HEADER_BAR_SIZE.y / 2.0),
                )
                .extend(101.0),
            ),
            ..default()
        },
        PlayerGameStopwatch { ..default() },
    ));
}

fn update_player_game_stopwatch(
    mut player_game_stopwatch_text_query: Query<(&mut Text, &mut PlayerGameStopwatch), With<PlayerGameStopwatch>>,
    time: Res<Time>,
) {
    for (mut text, mut stopwatch) in player_game_stopwatch_text_query.iter_mut() {
        stopwatch.timer.tick(time.delta());
        text.sections[0].value = get_elapsed_secs_as_a_parsed_string(stopwatch.timer.clone());
    }
}

fn get_elapsed_secs_as_a_parsed_string(timer: Stopwatch) -> String {
    let minutes = (timer.elapsed_secs() / 60.0) as i32;
    let seconds = (timer.elapsed_secs() % 60.0) as i32;
    let milliseconds = (timer.elapsed_secs().fract() * 100.0) as i32;
    format!("{:0>2}'{:0>2}''{:0>2}", minutes.to_string(), seconds.to_string(), milliseconds.to_string())
}
