use crate::prelude::*;

pub mod bombs;
pub mod consts;
pub mod monsters;
pub mod player_management;
pub mod tags;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, MonstersPlugin, GizmosPlugin, BombsPlugin));
    }
}
