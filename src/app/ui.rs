use bevy::text::Text2dBounds;
use crate::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);
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
    commands.spawn((Text2dBundle {
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
    PlayerGameStopwatch {
        elapsed_ms: 0
    })
    );
}
