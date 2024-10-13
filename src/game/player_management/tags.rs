use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct Player {
    pub held_bomb: Option<Entity>,
    pub score: u32,
}

#[derive(Component, Debug)]
pub struct PlayerMonsterCollider {
    pub radius: f32,
    pub colliding_monsters: Vec<Entity>,
}

impl PlayerMonsterCollider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_monsters: vec![],
        }
    }
}
