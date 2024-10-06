use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub enum BombRequest {
    PickUp,
    Release,
}

pub struct BombRequestPlugin;

impl Plugin for BombRequestPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BombRequest>();
    }
}
