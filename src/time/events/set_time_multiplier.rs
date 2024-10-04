use crate::prelude::*;

#[derive(Debug, Event, Clone, Copy)]
pub struct SetTimeMultiplier {
    pub multiplier_id: TimeMultiplierId,
    pub new_multiplier: f32,
    pub duration: f32,
}

pub struct SetTimeMultiplierPlugin;

impl Plugin for SetTimeMultiplierPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetTimeMultiplier>();
    }
}
