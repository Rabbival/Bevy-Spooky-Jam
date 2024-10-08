use bevy::window::WindowResolution;

use crate::prelude::*;

pub struct ScreenSetupPlugin;

impl Plugin for ScreenSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            WINDOW_SIZE_IN_PIXELS,
                            WINDOW_SIZE_IN_PIXELS / 4.0 * 3.0,
                        ),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(SCREEN_COLOR_BACKGROUND)
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.9,
        });
    }
}
