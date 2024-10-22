use crate::{prelude::*, read_single_field_variant};

pub struct PlayerVisualsPlugin;

impl Plugin for PlayerVisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            change_sprite_by_heading_direction.in_set(InputSystemSet::Handling),
        );
    }
}

fn change_sprite_by_heading_direction(
    mut player_request_listener: EventReader<PlayerRequest>,
    mut player_atlas_query: Query<&mut TextureAtlas, With<Player>>,
) {
    for normalized_move_direction in
        read_single_field_variant!(player_request_listener, PlayerRequest::Move)
    {
        for mut player_atlas in &mut player_atlas_query {
            let heading_direction = BasicDirection::closest(*normalized_move_direction);
            player_atlas.index = heading_direction.to_initial_frame_index();
        }
    }
}
