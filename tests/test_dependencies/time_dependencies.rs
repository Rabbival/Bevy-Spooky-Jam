use spooky_jam::prelude::*;
use std::time::Duration;

#[allow(dead_code)]
pub fn fast_forward(app: &mut App, time_to_advance_in_seconds: f32) {
    app.world_mut()
        .resource_mut::<Time>()
        .as_mut()
        .advance_by(Duration::from_secs_f32(time_to_advance_in_seconds));
}

#[allow(dead_code)]
pub fn spawn_redundant_calculator(app: &mut App, policy: TimerCalculatorSetPolicy) -> Entity {
    direct_spawn_redundant_calculator(&mut app.world_mut().commands(), policy)
}

#[allow(dead_code)]
pub fn direct_spawn_redundant_calculator(
    commands: &mut Commands,
    policy: TimerCalculatorSetPolicy,
) -> Entity {
    commands
        .spawn(GoingEventValueCalculator::new(
            policy,
            ValueByInterpolation::new(0.0, 0.0, Interpolator::default()),
            TimerGoingEventType::ChangeTimeMultiplierSpeed,
        ))
        .id()
}
