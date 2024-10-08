use crate::{prelude::*, read_no_field_variant};

pub struct BombThrowingPlugin;

impl Plugin for BombThrowingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (listen_for_bomb_throwing_requests,).in_set(InputSystemSet::Handling),
        );
    }
}

fn listen_for_bomb_throwing_requests(
    mut player_request_listener: EventReader<PlayerRequest>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
) {
    for bomb_throw_request in
        read_no_field_variant!(player_request_listener, PlayerRequest::ThrowBomb)
    {
        //TODO: throw it in mouse direction
        //REMINDER: make player held bomb none and set the bomb's state
        //REMINEDER: I only need to query over held bombs
        //REMINDER: also need to remove bomb from player children

        time_multiplier_request_writer.send(SetTimeMultiplier {
            multiplier_id: TimeMultiplierId::GameTimeMultiplier,
            new_multiplier: 1.0,
            duration: SLOW_MOTION_KICK_IN_AND_OUT_TIME,
        });
    }
}
