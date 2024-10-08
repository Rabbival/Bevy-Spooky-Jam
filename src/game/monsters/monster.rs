use crate::prelude::*;

#[derive(Component, Default)]
pub struct Monster {
    pub hearing_ring_distance: f32,
    pub state: MonsterState,
    pub last_player_location_seen: Vec3,
    pub bombs_location_seen: Vec<Vec3>,
}
