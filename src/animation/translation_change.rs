use crate::{get_mut_entity_else_return, prelude::*};

pub struct TranslationChangePlugin;

impl Plugin for TranslationChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_translation_update_requests.in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn listen_for_translation_update_requests(
    mut event_reader: EventReader<TimerGoingEvent<Vec3>>,
    mut transforms: Query<&mut Transform>,
) {
    for event_from_timer in event_reader.read() {
        if let TimerGoingEventType::Move(MovementType::InDirectLine) = event_from_timer.event_type {
            let mut transform =
                get_mut_entity_else_return!(transforms, event_from_timer.entity, Transform);
            transform.translation += event_from_timer.value_delta;
        }
    }
}
