use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum UiAction {
    CloseGame,
    #[cfg(debug_assertions)]
    DebugKey,
}
