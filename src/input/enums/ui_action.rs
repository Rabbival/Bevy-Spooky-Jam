use crate::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum UiAction {
    ToggleMenu,
    #[cfg(debug_assertions)]
    DebugKey,
}
