use crate::prelude::*;

use bevy::text::Text2dBounds;
use bevy_mod_reqwest::*;
use serde::{Deserialize, Serialize};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, (spawn_ui, make_test_request));

        if FunctionalityOverride::DontUpdateUI.disabled() {
            app.add_systems(
                Update,
                (
                    update_player_game_stopwatch,
                    update_player_scoring,
                    update_high_score,
                    update_longest_run_text,
                ),
            );
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Placeholder {
    pub id: u32,
    title: String,
    body: String,
}

fn make_test_request(
    mut client: BevyReqwest,
    mut best_score_so_far_query: Query<&mut BestScoreSoFar>,
) {
    println!("before");
    let mut best_score = best_score_so_far_query.get_single_mut();
    println!("after");
    let url = "https://jsonplaceholder.typicode.com/posts/100";
    let reqwest_request = client.get(url).build().unwrap();

    client
        // Sends the created http request
        .send(reqwest_request)
        // The response from the http request can be reached using an observersystem
        .on_response(move |trigger: Trigger<ReqwestResponseEvent>| {
            let response = trigger.event();
            let data = response.as_str();
            let status = response.status();
            //let json = response.deserialize_json::<Placeholder>();
            // let headers = req.response_headers();
            match response.deserialize_json::<Placeholder>() {
                Ok(json) => {
                    // Update the best score with the ID from the response
                    //best_score.unwrap().0 = json.id as u32;
                    bevy::log::info!("Status: {status:?}, Data: {data:?}");

                }
                Err(e) => {
                    bevy::log::info!("Failed to deserialize JSON: {e:?}");
                }
            }

        })
        // In case of request error, it can be reached using an observersystem
        .on_error(|trigger: Trigger<ReqwestErrorEvent>| {
            let e = &trigger.event().0;
            bevy::log::info!("error: {e:?}");
        });
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
        DoNotDestroyOnRestart,
    ));
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
        DoNotDestroyOnRestart,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Score: 0000000",
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
                    (-WINDOW_SIZE_IN_PIXELS / 2.0) + 100.0,
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
                "Hi  Score: 0000000",
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
        BestScoreTextUi,
        DoNotDestroyOnRestart,
    ));
}

fn update_player_game_stopwatch(
    mut player_game_stopwatch_text_query: Query<(&mut Text, &mut PlayerGameStopwatchUi)>,
    time: Res<Time>,
) {
    for (mut text, mut stopwatch) in &mut player_game_stopwatch_text_query {
        stopwatch.timer.tick(time.delta());
        text.sections[0].value = seconds_elapsed_to_pretty_string(stopwatch.timer.elapsed_secs());
    }
}

fn update_longest_run_text(
    changed_longest_run_query: Query<&LongestSurvivedSoFar, Changed<LongestSurvivedSoFar>>,
    mut longest_run_text_query: Query<&mut Text, With<LongestSurvivedUi>>,
) {
    for longest_run_time in &changed_longest_run_query {
        for mut longest_run_text in &mut longest_run_text_query {
            longest_run_text.sections[0].value =
                String::from("Longest: ") + &seconds_elapsed_to_pretty_string(longest_run_time.0);
        }
    }
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
                format!("Hi  Score: {:0>7}", best_score.0.to_string());
        }
    }
}

fn seconds_elapsed_to_pretty_string(seconds_elapsed: f32) -> String {
    let minutes = (seconds_elapsed / 60.0) as i32;
    let seconds = (seconds_elapsed % 60.0) as i32;
    let milliseconds = (seconds_elapsed.fract() * 100.0) as i32;
    format!(
        "{:0>2}'{:0>2}''{:0>2}",
        minutes.to_string(),
        seconds.to_string(),
        milliseconds.to_string()
    )
}
