use crate::{prelude::*, read_no_field_variant};

pub struct RespawnerPlugin;

impl Plugin for RespawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_invisible_again_screen)
            .add_systems(Update, show_again_and_respawn_world);
    }
}

fn spawn_invisible_again_screen(images: ResMut<SpritesAtlas>, mut commands: Commands) {
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
                CAMERA_Z_LAYER - 1.0,
            ),
            ..default()
        },
        AffectingTimerCalculators::default(),
        AgainScreen,
    ));
}

fn show_again_and_respawn_world(
    mut game_over_listener: EventReader<GameEvent>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
    again_screens: Query<(Entity, &Sprite), With<AgainScreen>>,
    mut timer_fire_writer: EventWriter<TimerFireRequest>,
    mut commands: Commands,
) {
    for _game_over in read_no_field_variant!(game_over_listener, GameEvent::GameOver) {
        for (again_screen_entity, again_screen_sprite) in &again_screens {
            let current_alpha = again_screen_sprite.color.alpha();
            let fade_in_timer =
                again_fade_timer(true, again_screen_entity, current_alpha, &mut commands);
            let fade_out_timer =
                again_fade_timer(false, again_screen_entity, current_alpha, &mut commands);
            if let Err(sequence_error) =
                TimerSequence::spawn_non_looping_sequence_and_fire_first_timer(
                    &mut timer_fire_writer,
                    &vec![fade_in_timer, fade_out_timer],
                    &mut commands,
                )
            {
                print_error(sequence_error, vec![LogCategory::RequestNotFulfilled]);
            }
            time_multiplier_request_writer.send(SetTimeMultiplier {
                multiplier_id: TimeMultiplierId::GameTimeMultiplier,
                new_multiplier: MULTIPLIER_WHEN_SLOW_MOTION,
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
