#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use bevy_light_2d::plugin::Light2dPlugin;

use crate::prelude::*;
use std::env;

pub mod app_events;
pub mod app_state;
pub mod app_state_manager;
pub mod assets_loader;
pub mod consts;
pub mod generic_plugins;
pub mod main_camera;
pub mod screen_setup;
pub mod tags;

#[bevy_main]
pub fn main() {
    let mut disable_output_log_file: bool = false;
    let args: Vec<String> = env::args().collect();
    if args.contains(&"DISABLE_OUTPUT_LOG_FILE".to_string()) {
        disable_output_log_file = true;
    }
    let mut app = App::new();
    app
        //bevy basics
        .add_plugins((ScreenSetupPlugin, Light2dPlugin))
        //costume
        .add_plugins((
            SystemSetsPlugin,
            InputPlugin,
            MainCameraPlugin,
            CustomAnimationPlugin,
            GamePlugin,
            AssetsLoaderPlugin,
            TimePlugin,
            LateDespawnerPlugin,
            UiPlugin,
            GameAudioPlugin,
            AppStateManagerPlugin,
            AppEventsPlugin,
            #[cfg(debug_assertions)]
            DebugPlugin,
        ))
        //generic plugins (type registration, for generic events for example)
        .add_plugins(GenericPlugins);

    app.run();
}
