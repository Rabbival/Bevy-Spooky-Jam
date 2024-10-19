use crate::{prelude::*, read_no_field_variant};

pub struct BombThrowingPlugin;

impl Plugin for BombThrowingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_bomb_throwing_requests.in_set(InputSystemSet::Handling),
        );
    }
}

fn listen_for_bomb_throwing_requests(
    mut player_request_listener: EventReader<PlayerRequest>,
    mut timer_fire_request_writer: EventWriter<TimerFireRequest>,
    mut time_multiplier_request_writer: EventWriter<SetTimeMultiplier>,
    mut player_query: Query<(&mut Player, &Transform), Without<Bomb>>,
    mut bomb_query: Query<(&mut Bomb, &mut Transform), Without<Player>>,
    cursor_world_position: Res<CursorWorldPosition>,
    mut commands: Commands,
    mut sounds_event_writer: EventWriter<SoundEvent>,
) {
    for _bomb_throw_request in
        read_no_field_variant!(player_request_listener, PlayerRequest::ThrowBomb)
    {
        for (mut player, player_transform) in &mut player_query {
            if let Some(bomb_entity) = player.held_bomb.take() {
                if let Ok((mut bomb, mut bomb_transform)) = bomb_query.get_mut(bomb_entity) {
                    disconnect_bomb_from_player(
                        player_transform,
                        bomb_entity,
                        &mut bomb_transform,
                        &mut bomb,
                        &mut commands,
                    );
                    throw_bomb(
                        &mut timer_fire_request_writer,
                        bomb_entity,
                        &bomb_transform,
                        cursor_world_position.0,
                        &mut commands,
                    );
                    sounds_event_writer.send(SoundEvent::BombThrowEvent);
                }
                time_multiplier_request_writer.send(SetTimeMultiplier {
                    multiplier_id: TimeMultiplierId::GameTimeMultiplier,
                    new_multiplier: 1.0,
                    duration: SLOW_MOTION_KICK_IN_AND_OUT_TIME,
                });
            }
        }
    }
}

fn disconnect_bomb_from_player(
    player_transform: &Transform,
    bomb_entity: Entity,
    bomb_transform: &mut Transform,
    bomb: &mut Bomb,
    commands: &mut Commands,
) {
    bomb.state = BombState::PostHeld;
    commands.entity(bomb_entity).remove::<Parent>();
    bomb_transform.translation += player_transform.translation; //now its transform is no longer relative to the player
}

fn throw_bomb(
    timer_fire_request_writer: &mut EventWriter<TimerFireRequest>,
    bomb_entity: Entity,
    bomb_transform: &Transform,
    cursor_position: Vec2,
    commands: &mut Commands,
) {
    let throw_value_calculator = bomb_throw_calculator(bomb_transform, cursor_position, commands);
    timer_fire_request_writer.send(TimerFireRequest {
        timer: EmittingTimer::new(
            vec![TimerAffectedEntity {
                affected_entity: bomb_entity,
                value_calculator_entity: Some(throw_value_calculator),
            }],
            vec![TimeMultiplierId::GameTimeMultiplier],
            bomb_transform
                .translation
                .distance(cursor_position.extend(Z_LAYER_BOMB))
                / BOMB_THROWING_SPEED,
            TimerDoneEventType::Nothing,
        ),
        parent_sequence: None,
    });
}

fn bomb_throw_calculator(
    bomb_transform: &Transform,
    cursor_position: Vec2,
    commands: &mut Commands,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            TimerCalculatorSetPolicy::AppendToTimersOfType,
            ValueByInterpolation::from_goal_and_current(
                bomb_transform.translation,
                cursor_position.extend(Z_LAYER_BOMB),
                Interpolator::default(),
            ),
            TimerGoingEventType::Move(MovementType::InDirectLine),
        ))
        .id()
}
