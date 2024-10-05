use crate::prelude::*;

#[derive(Resource)]
pub struct PlayerInputMap(pub InputMap<PlayerAction>);

pub struct PlayerInputMapPlugin;

impl Plugin for PlayerInputMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
            .insert_resource::<PlayerInputMap>(PlayerInputMap(InputMap::new([
                (PlayerAction::Move(BasicDirection::Up), KeyCode::ArrowUp),
                (
                    PlayerAction::Move(BasicDirection::Right),
                    KeyCode::ArrowRight,
                ),
                (PlayerAction::Move(BasicDirection::Down), KeyCode::ArrowDown),
                (PlayerAction::Move(BasicDirection::Left), KeyCode::ArrowLeft),
                (PlayerAction::PickUpOrReleaseBomb, KeyCode::Space),
            ])));
    }
}
