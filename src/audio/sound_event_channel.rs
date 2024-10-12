use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct BombExplodeSoundEvent;

pub struct SoundEventPlugin;

impl Plugin for SoundEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BombExplodeSoundEvent>();
    }
}
