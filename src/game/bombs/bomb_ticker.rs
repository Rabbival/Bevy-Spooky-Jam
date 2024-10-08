use crate::prelude::*;

pub struct BombTickerPlugin;

impl Plugin for BombTickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_for_bomb_tick_update);
    }
}

fn listen_for_bomb_tick_update(mut timer_going_event_reader: EventReader<TimerGoingEvent<f32>>) {
    for timer_going_event in timer_going_event_reader.read() {
        // TimerGoingEventType::BombCountdown
        //TODO: update bomb if the floored number recived is smaller than its current, make sure to also update the text
    }
}
