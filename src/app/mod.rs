#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use crate::prelude::*;
use bevy_light_2d::prelude::*;
use std::env;

pub mod assets_loader;
pub mod consts;
pub mod generic_plugins;
pub mod main_camera;
pub mod screen_setup;
pub mod tags;
pub mod ui;

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
        ))
        //generic plugins (type registration, for generic events for example)
        .add_plugins(GenericPlugins);

    if disable_output_log_file && !LOG_CATEGORYS_TO_APPEND_TO_SESSION_LOG.is_empty() {
        app.add_plugins(GameSessionLogPlugin);
    }

    app.run();
}
