use bevy::text::Text2dBounds;

use crate::{prelude::*, read_no_field_variant};

pub struct RespawnerPlugin;

impl Plugin for RespawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_invisible_again_screen)
            .add_systems(
                Update,
                (
                    show_again_and_respawn_world,
                    hide_again.in_set(GameRestartSystemSet::Spawning),
                ),
            );
    }
}

fn spawn_invisible_again_screen(
    images: ResMut<StaticImageHandles>,
    text_fonts_resource: ResMut<TextFonts>,
    mut commands: Commands,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgba(0.96, 0.96, 0.96, 0.0),
                ..default()
            },
            texture: images.again_screen_handle.clone(),
            transform: Transform::from_xyz(
                0.0,
                TOP_UI_HEADER_BAR_HEIGHT / 2.0,
                CAMERA_Z_LAYER - 10.0,
            ),
            ..default()
        },
        DoNotDestroyOnRestart,
        AffectingTimerCalculators::default(),
        AgainScreen,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Hi  Score: 0000000",
                TextStyle {
                    font: text_fonts_resource.kenny_high_square_handle.clone(),
                    font_size: 80.0,
                    color: Color::srgba(0.9, 0.9, 0.9, 0.0),
                },
            )
            .with_justify(JustifyText::Left),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(WINDOW_SIZE_IN_PIXELS, WINDOW_SIZE_IN_PIXELS / 4.0),
            },
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -(WINDOW_SIZE_IN_PIXELS * 4.0 / 10.0) + TOP_UI_HEADER_BAR_HEIGHT,
                CAMERA_Z_LAYER - 9.0,
            )),
            ..default()
        },
        AffectingTimerCalculators::default(),
        BestScoreTextUi,
        DoNotDestroyOnRestart,
        AgainScreen,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "Longest: ",
                TextStyle {
                    font: text_fonts_resource.kenny_high_square_handle.clone(),
                    font_size: 80.0,
                    color: Color::srgba(0.9, 0.9, 0.9, 0.0),
                },
            )
            .with_justify(JustifyText::Left),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(WINDOW_SIZE_IN_PIXELS, WINDOW_SIZE_IN_PIXELS / 4.0),
            },
            transform: Transform::from_translation(Vec3::new(
                0.0,
                -(WINDOW_SIZE_IN_PIXELS * 4.0 / 10.0) - TOP_UI_HEADER_BAR_HEIGHT / 2.0,
                CAMERA_Z_LAYER - 9.0,
            )),
            ..default()
        },
        AffectingTimerCalculators::default(),
        LongestSurvivedUi,
        DoNotDestroyOnRestart,
        AgainScreen,
    ));
}

fn show_again_and_respawn_world(
    mut game_over_listener: EventReader<GameEvent>,
    mut timer_fire_writer: EventWriter<TimerFireRequest>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
    again_screens: Query<(Entity, Option<&Sprite>, Option<&Text>), With<AgainScreen>>,
    mut commands: Commands,
) {
    for _game_over in read_no_field_variant!(game_over_listener, GameEvent::GameOver) {
        for (again_screen_entity, maybe_sprite, maybe_text) in &again_screens {
            let current_alpha;
            if let Some(sprite) = maybe_sprite {
                current_alpha = sprite.color.alpha();
            } else if let Some(text) = maybe_text {
                current_alpha = text.sections[0].style.color.alpha();
            } else {
                continue;
            }
            let fade_in_timer =
                again_fade_timer(true, again_screen_entity, current_alpha, &mut commands);
            timer_fire_writer.send(TimerFireRequest {
                timer: fade_in_timer,
                parent_sequence: None,
            });
            time_multiplier_request_writer.send(SetTimeMultiplier {
                multiplier_id: TimeMultiplierId::GameTimeMultiplier,
                new_multiplier: MULTIPLIER_WHEN_SLOW_MOTION,
                duration: AGAIN_SCREEN_FADE_TIME,
            });
        }
    }
}

fn hide_again(
    mut event_reader: EventReader<GameEvent>,
    mut timer_fire_writer: EventWriter<TimerFireRequest>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
    again_screens: Query<(Entity, Option<&Sprite>, Option<&Text>), With<AgainScreen>>,
    mut commands: Commands,
) {
    if read_no_field_variant!(event_reader, GameEvent::RestartGame).count() > 0 {
        for (again_screen_entity, maybe_sprite, maybe_text) in &again_screens {
            let current_alpha;
            if let Some(sprite) = maybe_sprite {
                current_alpha = sprite.color.alpha();
            } else if let Some(text) = maybe_text {
                current_alpha = text.sections[0].style.color.alpha();
            } else {
                continue;
            }
            let fade_out_timer =
                again_fade_timer(false, again_screen_entity, current_alpha, &mut commands);
            timer_fire_writer.send(TimerFireRequest {
                timer: fade_out_timer,
                parent_sequence: None,
            });
            time_multiplier_request_writer.send(SetTimeMultiplier {
                multiplier_id: TimeMultiplierId::GameTimeMultiplier,
                new_multiplier: 1.0,
                duration: AGAIN_SCREEN_FADE_TIME,
            });
        }
    }
}

fn again_fade_timer(
    fade_in: bool,
    again_screen_entity: Entity,
    current_alpha: f32,
    commands: &mut Commands,
) -> EmittingTimer {
    let calculator = spawn_again_fade_calculator(fade_in, current_alpha, commands);
    EmittingTimer::new(
        vec![TimerAffectedEntity {
            affected_entity: again_screen_entity,
            value_calculator_entity: Some(calculator),
        }],
        vec![TimeMultiplierId::RealTime],
        AGAIN_SCREEN_FADE_TIME,
        if fade_in {
            TimerDoneEventType::GameEvent(GameEvent::RestartGame)
        } else {
            TimerDoneEventType::Nothing
        },
    )
}

fn spawn_again_fade_calculator(
    fade_in: bool,
    current_alpha: f32,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::IgnoreNewIfAssigned,
            if fade_in {
                ValueByInterpolation::from_goal_and_current(
                    current_alpha,
                    1.0,
                    Interpolator::new(0.1),
                )
            } else {
                ValueByInterpolation::from_goal_and_current(1.0, 0.0, Interpolator::new(10.0))
            },
            TimerGoingEventType::SetAlpha,
        ))
        .id()
}
