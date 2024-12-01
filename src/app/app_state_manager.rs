use crate::prelude::*;
pub struct AppStateManagerPlugin;

impl Plugin for AppStateManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(Update, listen_to_game_initiation_request);
    }
}

fn listen_to_game_initiation_request(
    mut app_state_set_request_reader: EventReader<AppStateToggleRequest>,
    current_app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _set_request in app_state_set_request_reader.read() {
        let set_to = match current_app_state.get() {
            AppState::Game => AppState::Menu,
            AppState::Menu => AppState::Game,
        };
        next_state.set(set_to);
        info!("Changed app state to: {:?}", set_to);
    }
}
