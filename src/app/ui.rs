use crate::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui);
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn((SpriteBundle {
        sprite: Sprite {
            color: Color::srgba(1.0, 1.0, 1.0, 0.25),
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
    },));
}
