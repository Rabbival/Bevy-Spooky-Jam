use crate::prelude::*;

pub struct KeyboardInputHandlerPlugin;

impl Plugin for KeyboardInputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (listen_for_move_requests, slow_time_when_pressing_space)
                .in_set(InputSystemSet::Listening),
        );
    }
}

fn slow_time_when_pressing_space(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut event_writer: EventWriter<SetTimeMultiplier>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        event_writer.send(SetTimeMultiplier {
            multiplier_id: TimeMultiplierId::GameTimeMultiplier,
            new_multiplier: MULTIPLIER_WHEN_SLOW_MOTION,
            duration: 0.1,
        });
    }
    if keyboard_input.just_released(KeyCode::Space) {
        event_writer.send(SetTimeMultiplier {
            multiplier_id: TimeMultiplierId::GameTimeMultiplier,
            new_multiplier: 1.0,
            duration: 0.1,
        });
    }
}

fn listen_for_move_requests(keyboard_input: Res<ButtonInput<KeyCode>>) {
    let move_requests = keyboard_input
        .get_just_pressed()
        .map(BasicDirection::from_keycode);
    for _request in move_requests {
        //send an event
    }
}
