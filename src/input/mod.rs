use crate::prelude::*;

pub mod enums;
pub mod keyboard_input_handler;
pub mod mouse_input_handler;
pub mod player_input;
pub mod player_input_map;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            KeyboardInputHandlerPlugin,
            MouseInputHandlerPlugin,
            PlayerInputHandlerPlugin,
            PlayerInputMapPlugin,
        ));
    }
}
