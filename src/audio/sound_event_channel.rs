use crate::prelude::*;

pub struct SoundEventPlugin;

impl Plugin for SoundEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>();
    }
}
