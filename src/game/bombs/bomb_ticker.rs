use bevy_light_2d::light::PointLight2d;

use crate::prelude::*;
use std::num::ParseIntError;

pub struct BombTickerPlugin;

impl Plugin for BombTickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_bomb_tick_update.in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn listen_for_bomb_tick_update(
    mut timer_going_event_reader: EventReader<TimerGoingEvent<f32>>,
    mut bomb_query: Query<(&mut Bomb, &mut Sprite)>,
    mut text_query: Query<(&mut Text, &mut PointLight2d, &Parent)>,
    mut sounds_event_writer: EventWriter<SoundEvent>,
) {
    for timer_going_event in timer_going_event_reader.read() {
        if let TimerGoingEventType::BombCountdown = timer_going_event.event_type {
            if let Ok((mut bomb, mut bomb_sprite)) = bomb_query.get_mut(timer_going_event.entity) {
                if let BombState::PreHeld = bomb.state {
                    print_error(
                        BombError::AskedToTickAPreHeldBomb,
                        vec![LogCategory::RequestNotFulfilled],
                    );
                } else if let Err(_parse_error) = tick_bomb_and_update_text(
                    &mut bomb,
                    &mut bomb_sprite,
                    timer_going_event.value_delta,
                    timer_going_event.entity,
                    &mut text_query,
                    &mut sounds_event_writer,
                ) {
                    if let BombState::Exploded = bomb.state {
                    } else {
                        print_error(
                            BombError::CouldntParseTimerIntoInteger,
                            vec![LogCategory::RequestNotFulfilled],
                        );
                    }
                }
            } else {
                print_error(
                    EntityError::EntityNotInQuery("bomb to tick"),
                    vec![LogCategory::RequestNotFulfilled],
                );
            }
        }
    }
}

fn tick_bomb_and_update_text(
    bomb: &mut Bomb,
    bomb_sprite: &mut Sprite,
    time_delta: f32,
    bomb_entity: Entity,
    text_query: &mut Query<(&mut Text, &mut PointLight2d, &Parent)>,
    sounds_event_writer: &mut EventWriter<SoundEvent>,
) -> Result<(), ParseIntError> {
    bomb.time_until_explosion += time_delta;
    for (mut text, mut light, text_parent) in text_query {
        if text_parent.get() == bomb_entity {
            let text_value: usize = text.sections[0].value.parse()?;
            let ceiled_time_until_explosion = bomb.time_until_explosion.ceil() as usize;
            if text_value >= ceiled_time_until_explosion {
                text.sections[0].value = ceiled_time_until_explosion.to_string();
            }
            if ceiled_time_until_explosion <= ABOUT_TO_EXPLODE_TIME_CIEL {
                if !bomb.about_to_explode {
                    bomb.about_to_explode = true;
                    if let Some(bomb_colors) = bomb.to_colors() {
                        text.sections[0].style.color = bomb_colors.text;
                        light.color = bomb_colors.bomb;
                        light.intensity = BOMB_ABOUT_TO_EXPLODE_LIGHT_INTENSITY;
                        bomb_sprite.color = bomb_colors.bomb;
                    }
                }
                if bomb.about_to_explode
                    && ceiled_time_until_explosion < text_value
                    && 0 < ceiled_time_until_explosion
                {
                    sounds_event_writer.send(SoundEvent::BombTickEvent(
                        1.0 - (ceiled_time_until_explosion as f32 * 0.08),
                    ));
                }
            }
        }
    }
    Ok(())
}
