use crate::{prelude::*, read_no_field_variant};

#[derive(Debug, Clone, Copy)]
struct BombEntityAndDistance {
    bomb_entity: Entity,
    bomb_distance: f32,
}

pub struct BombPickingPlugin;

impl Plugin for BombPickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (listen_for_bomb_picking_attempts,).in_set(InputSystemSet::Handling),
        );
    }
}

fn listen_for_bomb_picking_attempts(
    mut player_request_listener: EventReader<PlayerRequest>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
    mut bomb_query: Query<(&mut Bomb, &mut Transform, Entity), Without<Player>>,
    mut player_query: Query<(&mut Player, &Transform, Entity), Without<Bomb>>,
    mut commands: Commands,
) {
    for _bomb_pickup_request in
        read_no_field_variant!(player_request_listener, PlayerRequest::PickUpBomb)
    {
        for (mut player, player_transform, player_entity) in &mut player_query {
            if let Some(bomb_entity) =
                try_getting_closest_bomb(player_transform.translation, &mut bomb_query)
            {
                make_player_hold_bomb(
                    player_entity,
                    &mut player,
                    player_transform,
                    bomb_entity,
                    &mut bomb_query,
                    &mut commands,
                );
                pull_bomb_and_slow_down_time(
                    bomb_entity,
                    bomb_query.get(bomb_entity).unwrap().1,
                    &mut timer_fire_request_writer,
                    &mut time_multiplier_request_writer,
                    &mut commands,
                );

                print_info(
                    format!("player picked up bomb entity: {:?}", bomb_entity),
                    vec![LogCategory::Player],
                );
            }
        }
    }
}

fn try_getting_closest_bomb(
    player_location: Vec3,
    bomb_query: &mut Query<(&mut Bomb, &mut Transform, Entity), Without<Player>>,
) -> Option<Entity> {
    let mut maybe_closest_bomb: Option<BombEntityAndDistance> = None;
    for (_, bomb_transform, bomb_entity) in bomb_query {
        let bomb_distance = calculate_distance_including_through_screen_border(
            player_location,
            bomb_transform.translation,
        )
        .distance;
        if bomb_distance < PLAYER_BOMB_PICKING_RANGE + BOMB_FULL_SIZE {
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

fn make_player_hold_bomb(
    player_entity: Entity,
    player: &mut Player,
    player_transform: &Transform,
    bomb_entity: Entity,
    bomb_query: &mut Query<(&mut Bomb, &mut Transform, Entity), Without<Player>>,
    commands: &mut Commands,
) {
    let (mut bomb, mut bomb_transform, _) = bomb_query.get_mut(bomb_entity).unwrap();
    bomb.bomb_state = BombState::Held;
    player.held_bomb = Some(bomb_entity);
    commands.entity(player_entity).push_children(&[bomb_entity]);
    bomb_transform.translation -= player_transform.translation; //now its transform is relative to the player
}

fn pull_bomb_and_slow_down_time(
    bomb_entity: Entity,
    bomb_transform: &Transform,
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    time_multiplier_request_writer: &mut EventWriter<SetTimeMultiplier>,
    commands: &mut Commands,
) {
    let bomb_pulling_calculator = spawn_bomb_puller_calculator(bomb_transform, commands);
    timer_fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: bomb_entity,
                value_calculator_entity: Some(bomb_pulling_calculator),
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            0.1,
            TimerDoneEventType::Nothing,
        ),
        parent_sequence: None,
    });
    time_multiplier_request_writer.send(SetTimeMultiplier {
        multiplier_id: TimeMultiplierId::GameTimeMultiplier,
        new_multiplier: MULTIPLIER_WHEN_SLOW_MOTION,
        duration: SLOW_MOTION_KICK_IN_AND_OUT_TIME,
    });
}

fn spawn_bomb_puller_calculator(bomb_transform: &Transform, commands: &mut Commands) -> Entity {
    let bomb_spot_relative_to_player = -bomb_transform.scale;
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::IgnoreNewIfAssigned,
            ValueByInterpolation::from_goal_and_current(
                Vec3::new(
                    bomb_transform.translation.x,
                    bomb_transform.translation.y,
                    0.0,
                ),
                Vec3::new(
                    bomb_spot_relative_to_player.x,
                    bomb_spot_relative_to_player.y,
                    0.0,
                ),
                Interpolator::new(0.6),
            ),
            TimerGoingEventType::Move(MovementType::InDirectLine),
        ))
        .id()
}