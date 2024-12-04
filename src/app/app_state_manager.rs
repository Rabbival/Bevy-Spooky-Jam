use crate::{prelude::*, read_no_field_variant};
pub struct AppStateManagerPlugin;

impl Plugin for AppStateManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_systems(
                Update,
                (
                    listen_to_app_state_toggle_requests,
                    listen_to_bomb_throwing_requests.run_if(in_state(AppState::Menu)),
                )
                    .in_set(InputSystemSet::Handling),
            )
            .add_systems(OnEnter(AppState::Menu), pause_spawners)
            .add_systems(OnExit(AppState::Menu), unpause_spawners);
    }
}

fn listen_to_app_state_toggle_requests(
    mut app_state_set_request_reader: EventReader<AppStateToggleRequest>,
    mut game_event_writer: EventWriter<GameEvent>,
    current_app_state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _set_request in app_state_set_request_reader.read() {
        let set_to = match current_app_state.get() {
            AppState::Game => AppState::Menu,
            AppState::Menu => AppState::Game,
        };
        set_app_state(&mut game_event_writer, set_to, &mut next_state);
    }
}

fn listen_to_bomb_throwing_requests(
    mut player_request_listener: EventReader<PlayerRequest>,
    mut game_event_writer: EventWriter<GameEvent>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    for _bomb_throw_request in
        read_no_field_variant!(player_request_listener, PlayerRequest::ThrowBomb)
    {
        set_app_state(&mut game_event_writer, AppState::Game, &mut next_state)
    }
}

fn set_app_state(
    game_event_writer: &mut EventWriter<GameEvent>,
    set_to: AppState,
    next_state: &mut ResMut<NextState<AppState>>,
) {
    next_state.set(set_to);
    if let AppState::Menu = set_to {
        game_event_writer.send(GameEvent::RestartGame);
    }
    info!("Changed app state to: {:?}", set_to);
}

fn pause_spawners(mut event_writer: EventWriter<SetTimeMultiplierOverridingValue>) {
    event_writer.send(SetTimeMultiplierOverridingValue {
        multiplier_id: TimeMultiplierId::EntitySpawnersTimeMultiplier,
        new_overriding_value: Some(0.0),
    });
}

fn unpause_spawners(mut event_writer: EventWriter<SetTimeMultiplierOverridingValue>) {
    event_writer.send(SetTimeMultiplierOverridingValue {
        multiplier_id: TimeMultiplierId::EntitySpawnersTimeMultiplier,
        new_overriding_value: None,
    });
}
