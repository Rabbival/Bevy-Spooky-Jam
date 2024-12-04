use crate::prelude::*;

pub mod consts;
pub mod instructions_text_manager;
pub mod tags;
pub mod ui_spawner;
pub mod ui_updater;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiSpawnerPlugin, InstructionsTextPlugin));
        if FunctionalityOverride::DontUpdateUI.disabled() {
            app.add_plugins(UiUpdaterPlugin);
        }
    }
}
