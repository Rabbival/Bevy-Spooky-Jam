use crate::prelude::*;

pub struct ExplodeInContactManagerPlugin;

impl Plugin for ExplodeInContactManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_bounding_circle_location,
                listen_for_scale_change_requests,
            ),
        );
    }
}

fn update_bounding_circle_location(
    mut explode_in_contact_query: Query<(&mut ExplodeInContact, &Transform), Changed<Transform>>,
) {
    for (mut circle_container, transform) in &mut explode_in_contact_query {
        circle_container.bounding_circle.center = transform.translation.truncate();
    }
}

fn listen_for_scale_change_requests(
    mut event_reader: EventReader<TimerGoingEvent<Vec3>>,
    mut explode_in_contact_query: Query<(&mut ExplodeInContact, Option<&Monster>)>,
) {
    for event_from_timer in event_reader.read() {
        if let TimerGoingEventType::Scale = event_from_timer.event_type {
            if let Ok((mut circle_container, maybe_monster)) =
                explode_in_contact_query.get_mut(event_from_timer.entity)
            {
                let mut delta = event_from_timer.value_delta.x;
                if maybe_monster.is_some() {
                    delta *= MONSTER_COLLIDER_RADIUS_FACTOR_WHEN_CHASING;
                }
                circle_container.bounding_circle.circle.radius += delta;
            }
        }
    }
}
