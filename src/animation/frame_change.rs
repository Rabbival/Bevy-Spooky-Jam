use crate::prelude::*;

pub struct FrameChangePlugin;

impl Plugin for FrameChangePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_for_frame_update_requests.in_set(TickingSystemSet::PostTicking),
        );
    }
}

fn listen_for_frame_update_requests(
    mut timer_done_event_reader: EventReader<TimerDoneEvent>,
    mut atlas_users_query: Query<&mut TextureAtlas>,
) {
    for done_event in timer_done_event_reader.read() {
        if let TimerDoneEventType::SetFrame(frame) = done_event.event_type {
            for entity in done_event
                .affected_entities
                .iter()
                .map(|timer_affected| timer_affected.affected_entity)
            {
                if let Ok(mut atlas) = atlas_users_query.get_mut(entity) {
                    atlas.index = frame;
                } else {
                    print_error(
                        EntityError::EntityNotInQuery(
                            "couldn't fetch texture atlas from query on frame update function",
                        ),
                        vec![LogCategory::RequestNotFulfilled],
                    );
                }
            }
        }
    }
}
