use crate::prelude::*;

pub mod consts;
pub mod ui_spawner;
pub mod ui_updater;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiSpawnerPlugin);
        if FunctionalityOverride::DontUpdateUI.disabled() {
            app.add_plugins(UiUpdaterPlugin);
        }
    }
}
