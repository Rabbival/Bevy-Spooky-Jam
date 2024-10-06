use crate::prelude::*;

pub mod player_input_map;
pub mod ui_input_map;

pub struct InputMapsPlugin;

impl Plugin for InputMapsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((PlayerInputMapPlugin, UiInputMapPlugin));
    }
}
