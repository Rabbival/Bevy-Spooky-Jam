use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub enum PlayerRequest {
    Move(BasicDirection),
    PickUpBomb,
    ThrowBomb,
}

pub struct PlayerRequestPlugin;

impl Plugin for PlayerRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerRequest>();
    }
}
