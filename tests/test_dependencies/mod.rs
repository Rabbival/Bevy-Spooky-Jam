use spooky_jam::prelude::*;

pub mod time_dependencies;

pub fn get_app_with_resources_and_events() -> App {
    let mut app = App::new();
    app.init_resource::<Time>()
        .init_resource::<Assets<Mesh>>()
        .init_resource::<Assets<ColorMaterial>>()
        .add_event::<ValueCalculatorRequest>()
        .add_event::<TimerDoneEvent>()
        .add_event::<TimerGoingEvent<f32>>()
        .add_event::<TimerGoingEvent<Vec2>>()
        .add_event::<TimerGoingEvent<Vec3>>()
        .add_event::<TimerGoingEvent<Quat>>()
        .add_event::<TimerFireRequest>()
        .add_event::<RemoveFromTimerAffectedEntities>()
        .add_event::<CalculateAndSendGoingEvent>()
        .add_event::<UpdateAffectedEntitiesAfterTimerBirth>();
    app
}

#[allow(dead_code)]
pub fn spawn_empty_entity(app: &mut App) -> Entity {
    direct_spawn_empty_entity(&mut app.world_mut().commands())
}

#[allow(dead_code)]
pub fn direct_spawn_empty_entity(commands: &mut Commands) -> Entity {
    commands.spawn(AffectingTimerCalculators::default()).id()
}
