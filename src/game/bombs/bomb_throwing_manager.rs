use crate::{prelude::*, read_no_field_variant};

#[derive(Debug, Clone, Copy)]
struct BombEntityAndDistance {
    bomb_entity: Entity,
    bomb_distance: f32,
}

pub struct BombThrowingManagerPlugin;

impl Plugin for BombThrowingManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (listen_for_bomb_throwing_attempts, listen_for_bomb_releasing)
                .in_set(InputSystemSet::Handling),
        );
    }
}

fn listen_for_bomb_throwing_attempts(
    mut player_request_listener: EventReader<PlayerRequest>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
    mut bomb_query: Query<(&mut Bomb, &Transform, Entity)>,
    player_query: Query<&Transform, With<Player>>,
) {
    for _bomb_pickup_request in
        read_no_field_variant!(player_request_listener, PlayerRequest::PickUpBomb)
    {
        for player_transform in &player_query {
            if let Some(bomb_entity) =
                try_getting_closest_bomb(player_transform.translation, &mut bomb_query)
            {
                bomb_query.get_mut(bomb_entity).unwrap().0.bomb_state = BombState::Held;
                pick_bomb_up(bomb_entity, &mut time_multiplier_request_writer);
                print_info(
                    &format!("player picked up bomb entity: {:?}", bomb_entity),
                    vec![LogCategory::Player],
                );
            }
        }
    }
}

fn try_getting_closest_bomb(
    player_location: Vec3,
    bomb_query: &mut Query<(&mut Bomb, &Transform, Entity)>,
) -> Option<Entity> {
    let mut maybe_closest_bomb: Option<BombEntityAndDistance> = None;
    for (_, bomb_transform, bomb_entity) in bomb_query {
        let bomb_distance = calculate_distance_including_through_screen_border(
            player_location,
            bomb_transform.translation,
        )
        .distance;
        if bomb_distance < PLAYER_BOMB_PICKING_RANGE {
            let bomb_properties = Some(BombEntityAndDistance {
                bomb_entity,
                bomb_distance,
            });
            match maybe_closest_bomb {
                Some(closest_bomb_so_far) => {
                    if bomb_distance < closest_bomb_so_far.bomb_distance {
                        maybe_closest_bomb = bomb_properties;
                    }
                }
                None => {
                    maybe_closest_bomb = bomb_properties;
                }
            }
        }
    }
    maybe_closest_bomb.map(|closest_bomb| closest_bomb.bomb_entity)
}

fn pick_bomb_up(
    bomb_entity: Entity,
    time_multiplier_request_writer: &mut EventWriter<SetTimeMultiplier>,
) {
    //TODO
    //Pull bomb towards player (timer)
    //Set player bomb to Some(bomb_entity)
    //* Should the player be slower while holding a bomb because it's heavy?
    //Attach bomb as child entity to player

    time_multiplier_request_writer.send(SetTimeMultiplier {
        multiplier_id: TimeMultiplierId::GameTimeMultiplier,
        new_multiplier: MULTIPLIER_WHEN_SLOW_MOTION,
        duration: SLOW_MOTION_KICK_IN_AND_OUT_TIME,
    });
}

fn listen_for_bomb_releasing(
    mut player_request_listener: EventReader<PlayerRequest>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
) {
    for bomb_throw_request in
        read_no_field_variant!(player_request_listener, PlayerRequest::ThrowBomb)
    {
        //TODO: throw it in mouse direction

        time_multiplier_request_writer.send(SetTimeMultiplier {
            multiplier_id: TimeMultiplierId::GameTimeMultiplier,
            new_multiplier: 1.0,
            duration: SLOW_MOTION_KICK_IN_AND_OUT_TIME,
        });
    }
}
