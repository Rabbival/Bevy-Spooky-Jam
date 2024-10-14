use crate::prelude::*;

pub mod main_path_initiation;
pub mod stray_path_ender;
pub mod stray_path_updater;

pub struct MonsterPathUpdatingPlugin;

impl Plugin for MonsterPathUpdatingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MonsterStrayPathUpdaterPlugin,
            MonsterStrayPathEnderPlugin,
            MonsterMainPathInitiationPlugin,
        ));
    }
}
