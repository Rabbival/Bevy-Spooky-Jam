use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum UiAction {
    CloseGame,
    RestartGame,
    #[cfg(debug_assertions)]
    DebugKey,
}
