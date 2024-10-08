use crate::prelude::*;

pub struct ExplosionManagerPlugin;

impl Plugin for ExplosionManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, listen_for_done_bombs);
    }
}

fn listen_for_done_bombs(mut timer_done_reader: EventReader<TimerDoneEvent>) {
    for done_timer in timer_done_reader.read() {
        // TimerDoneEventType::ExplodeInRadius(())
        //TODO: destroy hit entities
    }
}
