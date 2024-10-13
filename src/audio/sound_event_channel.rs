use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub enum SoundEvent {
    BombExplodeSoundEvent,
    BombPickUpEvent,
    BombThrowEvent,
    BombTickEvent(f32),
    MonsterBattleCry,
    MonsterDeathCry,
}

pub struct SoundEventPlugin;

impl Plugin for SoundEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SoundEvent>();
    }
}
