use crate::prelude::*;

pub struct UiInputHandlerPlugin;

impl Plugin for UiInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ui_input_listener)
            .add_systems(
                Update,
                listen_for_ui_just_pressed_controls.in_set(InputSystemSet::Listening),
            );
    }
}

fn spawn_ui_input_listener(ui_input_map: Res<UiInputMap>, mut commands: Commands) {
    commands.spawn(InputManagerBundle::<UiAction> {
        action_state: ActionState::default(),
        input_map: ui_input_map.0.clone(),
    });
}

fn listen_for_ui_just_pressed_controls(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut player_query: Query<&ActionState<UiAction>>,
) {
    for action_map in &mut player_query {
        for action in action_map.get_just_pressed() {
            match action {
                UiAction::CloseGame => {
                    info!("app exit");
                    app_exit_event_writer.send(AppExit::Success);
                }
            }
        }
    }
}