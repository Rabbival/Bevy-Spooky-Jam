use crate::prelude::*;

#[derive(Event, Debug, Clone, Copy, PartialEq)]
pub struct BombExploded {
    pub location: Vec3,
    pub monster_hit_count: usize,
    pub hit_player: bool,
}

pub struct BombEventsPlugin;

impl Plugin for BombEventsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<BombExploded>();
    }
}
