use crate::prelude::*;

#[derive(Debug, Component, Clone, Copy, PartialEq)]
pub struct EmittingTimer {
    pub affected_entities: VecBasedArray<TimerAffectedEntity, TIMER_MAX_ASSIGNED_ENTITIES>,
    pub time_multipliers: VecBasedArray<TimeMultiplierId, TIMER_MAX_ASSIGNED_MULTIPLIERS>,
    duration: f32,
    pub send_once_done: TimerDoneEventType,
    elapsed_time: f32,
    normalized_progress: f32,
}

impl EmittingTimer {
    pub fn new(
        affected_entities_vec: Vec<TimerAffectedEntity>,
        time_multipliers_vec: Vec<TimeMultiplierId>,
        duration: f32,
        send_once_done: TimerDoneEventType,
    ) -> Self {
        let clamped_duration =
            clamp_and_notify(duration, A_MILLISECOND_IN_SECONDS, AN_HOUR_IN_SECONDS);
        let affected_entities_array = VecBasedArray::new(affected_entities_vec);
        let time_multipliers_array = VecBasedArray::new(time_multipliers_vec);
        Self {
            affected_entities: affected_entities_array,
            time_multipliers: time_multipliers_array,
            duration: clamped_duration,
            send_once_done,
            elapsed_time: 0.0,
            normalized_progress: 0.0,
        }
    }

    pub fn affected_entities_iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.affected_entities.affected_entities_iter()
    }

    pub fn calculator_entities_iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.affected_entities.calculator_entities_iter()
    }

    pub fn finished(&self) -> bool {
        self.normalized_progress >= 1.0
    }

    pub fn tick_and_get_normalized_progress(&mut self, processed_time: f32) -> Option<f32> {
        if processed_time > 0.0 && !self.finished() {
            self.elapsed_time += processed_time;
            self.update_normalized_progress();
            Some(self.normalized_progress)
        } else {
            None
        }
    }

    fn update_normalized_progress(&mut self) {
        self.normalized_progress = (self.elapsed_time / self.duration).min(1.0);
    }
}
