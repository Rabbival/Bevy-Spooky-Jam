use spooky_jam::prelude::*;

mod test_dependencies;

#[test]
fn orb_spawn_and_despawn_single_request_test() {
    let orb_count_before_time_passed;
    let mut app = test_dependencies::get_app_with_resources_and_events();
    app.add_systems(
        Update,
        (
            spawn_orb,
            collect_all_orbs,
            listen_for_emitting_timer_firing_requests,
            listen_for_update_affected_entities_after_timer_birth_requests::<Vec3>,
            tick_emitting_timers,
            calculate_value_and_send_going_event::<Vec3>,
            listen_for_despawn_requests_from_timers,
        )
            .chain(),
    );
    request_max_orb_spawns(&mut app, Vec2::default());
    fire_orb_collection_request(&mut app, Vec2::default());

    app.update();
    orb_count_before_time_passed = count_orbs_in_world(&mut app);
    test_dependencies::time_dependencies::fast_forward(&mut app, ORB_COLLECTION_TIME);
    app.update();

    assert_eq!(orb_count_before_time_passed, ORB_MAX_COUNT);
    assert_eq!(count_orbs_in_world(&mut app), 0);
}

#[test]
fn orb_spawn_and_despawn_multiple_requests_test() {
    const COLLECTION_REQUEST_COUNT: usize = 8;
    const ORB_COLLECTION_DESTINATION: Vec2 = Vec2::new(100.0, 100.0);
    let mut orb_counts = vec![];
    let mut app = test_dependencies::get_app_with_resources_and_events();
    app.add_systems(
        Update,
        (
            spawn_orb,
            collect_all_orbs,
            listen_for_emitting_timer_firing_requests,
            listen_for_update_affected_entities_after_timer_birth_requests::<Vec3>,
            tick_emitting_timers,
            calculate_value_and_send_going_event::<Vec3>,
            listen_for_despawn_requests_from_timers,
        )
            .chain(),
    );
    request_max_orb_spawns(&mut app, Vec2::default());

    for _ in 0..COLLECTION_REQUEST_COUNT {
        fire_orb_collection_request(&mut app, ORB_COLLECTION_DESTINATION);
        app.update();
        test_dependencies::time_dependencies::fast_forward(
            &mut app,
            ORB_COLLECTION_TIME / (COLLECTION_REQUEST_COUNT as f32),
        );
        orb_counts.push(count_orbs_in_world(&mut app));
    }
    app.update();

    assert_eq!(count_orbs_in_world(&mut app), 0);
    for orb_count in orb_counts {
        assert_eq!(orb_count, ORB_MAX_COUNT);
    }
}

fn fire_orb_collection_request(app: &mut App, collect_to: Vec2) {
    app.world_mut()
        .resource_mut::<Events<OrbEvent>>()
        .send(OrbEvent::CollectAllOrbs(collect_to));
}

fn count_orbs_in_world(app: &mut App) -> usize {
    app.world_mut().query::<&Orb>().iter(app.world()).len()
}

fn request_max_orb_spawns(app: &mut App, location: Vec2) {
    let mut orb_event_channel_writer = app.world_mut().resource_mut::<Events<OrbEvent>>();
    for _ in 0..ORB_MAX_COUNT {
        orb_event_channel_writer.send(OrbEvent::SpawnOrb(location));
    }
}
