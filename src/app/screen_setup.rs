use bevy::{asset::AssetMetaCheck, window::WindowResolution};

use crate::prelude::*;

pub struct ScreenSetupPlugin;

impl Plugin for ScreenSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Soul Bomb Monster Hunt".to_string(),
                        canvas: Some("#bevy".to_string()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: true,
                        resolution: WindowResolution::new(
                            WINDOW_SIZE_IN_PIXELS,
                            WINDOW_SIZE_IN_PIXELS + TOP_UI_HEADER_BAR_HEIGHT,
                        ),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_resource(SCREEN_COLOR_BACKGROUND);
    }
}
