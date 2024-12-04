use bevy::text::Text2dBounds;

use crate::{prelude::*, read_no_field_variant};

pub struct InstructionsTextPlugin;

impl Plugin for InstructionsTextPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_instructions_text)
            .add_systems(
                Update,
                listen_to_bomb_pick_up_event.run_if(in_state(AppState::Menu)),
            )
            .add_systems(OnExit(AppState::Menu), despawn_rest_of_text);
    }
}

fn spawn_instructions_text(text_fonts_resource: Res<TextFonts>, mut commands: Commands) {
    spawn_instruction_text_in_location(
        "Hold the Left Mouse Button",
        InstructionsTextUi::Hold,
        Vec3::new(
            -WINDOW_SIZE_IN_PIXELS / 8.0,
            PLAYER_BOMB_PICKING_RANGE * 3.0,
            CAMERA_Z_LAYER - 9.0,
        ),
        &text_fonts_resource,
        &mut commands,
    );
    spawn_instruction_text_in_location(
        "Aim",
        InstructionsTextUi::Aim,
        Vec3::new(
            -WINDOW_SIZE_IN_PIXELS / 1.88,
            -PLAYER_BOMB_PICKING_RANGE * 2.0,
            CAMERA_Z_LAYER - 9.0,
        ),
        &text_fonts_resource,
        &mut commands,
    );
    spawn_instruction_text_in_location(
        "Then,  release.",
        InstructionsTextUi::Release,
        Vec3::new(
            -WINDOW_SIZE_IN_PIXELS / 7.0,
            -PLAYER_BOMB_PICKING_RANGE * 4.0,
            CAMERA_Z_LAYER - 9.0,
        ),
        &text_fonts_resource,
        &mut commands,
    );
}

fn spawn_instruction_text_in_location(
    instruction_text: &str,
    text_tag: InstructionsTextUi,
    position: Vec3,
    text_fonts_resource: &TextFonts,
    commands: &mut Commands,
) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                instruction_text,
                TextStyle {
                    font: text_fonts_resource.kenny_high_square_handle.clone(),
                    font_size: 60.0,
                    color: Color::srgba(0.9, 0.9, 0.9, 1.0),
                },
            )
            .with_justify(JustifyText::Center),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(WINDOW_SIZE_IN_PIXELS, WINDOW_SIZE_IN_PIXELS / 4.0),
            },
            transform: Transform::from_translation(position),
            ..default()
        },
        AffectingTimerCalculators::default(),
        text_tag,
    ));
}

fn listen_to_bomb_pick_up_event(
    mut player_request_listener: EventReader<PlayerRequest>,
    mut timer_fire_writer: EventWriter<TimerFireRequest>,
    instructions_text_query: Query<(&InstructionsTextUi, &Text, Entity)>,
    mut commands: Commands,
) {
    if read_no_field_variant!(player_request_listener, PlayerRequest::PickUpBomb).count() > 0 {
        for (text_type, text, entity) in &instructions_text_query {
            if let InstructionsTextUi::Hold = text_type {
                fade_out_and_despawn(&mut timer_fire_writer, &text, entity, &mut commands);
            }
        }
    }
}

fn despawn_rest_of_text(
    mut timer_fire_writer: EventWriter<TimerFireRequest>,
    instructions_text_query: Query<(&InstructionsTextUi, &Text, Entity)>,
    mut commands: Commands,
) {
    for (text_type, text, entity) in &instructions_text_query {
        match text_type {
            InstructionsTextUi::Aim | InstructionsTextUi::Release => {
                fade_out_and_despawn(&mut timer_fire_writer, &text, entity, &mut commands);
            }
            _ => {}
        }
    }
}

fn fade_out_and_despawn(
    timer_fire_writer: &mut EventWriter<TimerFireRequest>,
    text: &Text,
    entity: Entity,
    commands: &mut Commands,
) {
    let current_alpha = text.sections[0].style.color.alpha();
    let fade_in_timer = fade_out_timer(entity, current_alpha, commands);
    timer_fire_writer.send(TimerFireRequest {
        timer: fade_in_timer,
        parent_sequence: None,
    });
}

fn fade_out_timer(
    text_entity: Entity,
    current_alpha: f32,
    commands: &mut Commands,
) -> EmittingTimer {
    let calculator = spawn_fade_out_calculator(current_alpha, commands);
    EmittingTimer::new(
        vec![TimerAffectedEntity {
            affected_entity: text_entity,
            value_calculator_entity: Some(calculator),
        }],
        vec![TimeMultiplierId::RealTime],
        INSTRUCTIONS_TEXT_FADE_OUT_TIME,
        TimerDoneEventType::DespawnAffectedEntities(DespawnPolicy::DespawnSelf),
    )
}

fn spawn_fade_out_calculator(current_alpha: f32, commands: &mut Commands) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::IgnoreNewIfAssigned,
            ValueByInterpolation::from_goal_and_current(current_alpha, 0.0, Interpolator::new(0.2)),
            TimerGoingEventType::SetAlpha,
        ))
        .id()
}
