use crate::prelude::*;

pub mod enums;
pub mod input_maps;
pub mod mouse_input_handler;
pub mod player_input;
pub mod ui_input;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // MouseInputHandlerPlugin,
            PlayerInputHandlerPlugin,
            InputMapsPlugin,
            UiInputHandlerPlugin,
        ));
    }
}
