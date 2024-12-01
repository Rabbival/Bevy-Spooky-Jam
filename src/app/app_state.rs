use bevy::prelude::States;

#[derive(Debug, Default, States, Clone, Copy, Hash, PartialEq, Eq)]
pub enum AppState {
    Game,
    #[default]
    Menu,
}
