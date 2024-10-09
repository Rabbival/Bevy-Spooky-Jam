use crate::prelude::*;

#[derive(Component, Debug, Clone, Copy, PartialEq, Default)]
pub struct Player {
    pub held_bomb: Option<Entity>,
    pub score: u32,
}
