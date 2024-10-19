use crate::prelude::*;

#[derive(Resource)]
pub struct UiInputMap(pub InputMap<UiAction>);

pub struct UiInputMapPlugin;

impl Plugin for UiInputMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<UiAction>::default())
            .insert_resource::<UiInputMap>(UiInputMap(InputMap::new([
                (UiAction::CloseGame, KeyCode::Escape),
                (UiAction::RestartGame, KeyCode::KeyR),
                #[cfg(debug_assertions)]
                (UiAction::DebugKey, KeyCode::KeyK),
            ])));
    }
}
