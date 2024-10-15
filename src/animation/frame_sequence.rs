use crate::prelude::*;

pub struct FrameSequence(pub TimerSequence);

impl FrameSequence {
    pub fn looping_frame_sequence(
        affected_entities: Vec<Entity>,
        time_multipliers: Vec<TimeMultiplierId>,
        duration: f32,
        indexes: Vec<usize>,
    ) -> FrameSequence {
        FrameSequence(frame_timer_sequence(
            true,
            affected_entities,
            time_multipliers,
            duration,
            indexes,
        ))
    }

    pub fn non_looping_frame_sequence(
        affected_entities: Vec<Entity>,
        time_multipliers: Vec<TimeMultiplierId>,
        duration: f32,
        indexes: Vec<usize>,
    ) -> FrameSequence {
        FrameSequence(frame_timer_sequence(
            false,
            affected_entities,
            time_multipliers,
            duration,
            indexes,
        ))
    }
}

fn frame_timer_sequence(
    looping: bool,
    affected_entities: Vec<Entity>,
    time_multipliers: Vec<TimeMultiplierId>,
    duration: f32,
    indexes: Vec<usize>,
) -> TimerSequence {
    let timer_affected_entities: Vec<TimerAffectedEntity> = affected_entities
        .iter()
        .map(|&affected_entity| TimerAffectedEntity {
            affected_entity,
            value_calculator_entity: None,
        })
        .collect();
    let mut emitting_timers = vec![];
    for index in indexes {
        emitting_timers.push(EmittingTimer::new(
            timer_affected_entities.clone(),
            time_multipliers.clone(),
            duration,
            TimerDoneEventType::SetFrame(index),
        ));
    }
    if looping {
        TimerSequence::looping_sequence(&emitting_timers)
    } else {
        TimerSequence::non_looping_sequence(&emitting_timers)
    }
}
