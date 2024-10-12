use crate::prelude::Event;
use bevy::prelude::Component;

#[derive(Debug, Component)]
pub struct MusicLayer(pub u8);

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct SoundEvent {
    pub event: SoundEventEnum,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SoundEventEnum {
    BombExplodeSoundEvent,
    BombPickUpEvent,
    BombThrowEvent,
    BombTickEvent,
}
