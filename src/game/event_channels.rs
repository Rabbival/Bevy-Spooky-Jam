use crate::prelude::*;

#[derive(Debug, Event)]
pub enum OrbEvent{
    SpawnOrb(Vec2),
    CollectAllOrbs(Vec2),
}


pub struct GameEventChannelsPlugin;

impl Plugin for GameEventChannelsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OrbEvent>();
    }
}
