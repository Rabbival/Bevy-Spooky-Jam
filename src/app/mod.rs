#![allow(clippy::too_many_arguments, clippy::type_complexity)]

use crate::prelude::*;

pub mod assets_loader;
pub mod consts;
pub mod generic_plugins;
pub mod main_camera;
pub mod screen_setup;
pub mod tags;
pub mod ui;

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app
        //bevy basics
        .add_plugins(ScreenSetupPlugin)
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
        ))
        //generic plugins (type registration, for generic events for example)
        .add_plugins(GenericPlugins);

    if !LOG_CATEGORYS_TO_APPEND_TO_SESSION_LOG.is_empty() {
        app.add_plugins(GameSessionLogPlugin);
    }

    app.run();
}
