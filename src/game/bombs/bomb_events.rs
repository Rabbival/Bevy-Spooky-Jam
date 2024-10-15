use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct BombExploded {
    pub location: Vec3,
    pub hit_monster: bool,
}

pub struct BombEventsPlugin;

impl Plugin for BombEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BombExploded>();
    }
}
