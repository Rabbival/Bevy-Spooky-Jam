use crate::prelude::*;
use leafwing_input_manager::prelude::*;

pub mod keyboard_input_handler;
pub mod mouse_input_handler;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((KeyboardInputHandlerPlugin, MouseInputHandlerPlugin))
            .add_plugins(InputManagerPlugin::<PlayerAction>::default());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PlayerAction {
    MoveLeft,
    MoveUp,
    MoveRight,
    MoveDown,
    Fire,
}
