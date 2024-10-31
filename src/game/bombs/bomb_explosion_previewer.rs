use crate::prelude::*;

pub struct BombExplosionPreviewerPlugin;

impl Plugin for BombExplosionPreviewerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (toggle_bomb_explosion_preview, update_explosion_preview)
                .in_set(InputSystemSet::Handling),
        );
    }
}

fn update_explosion_preview() {
    //TODO: if bomb_picked_reader.not_empty or Changed<MousePosition>     something like that
}

fn toggle_bomb_explosion_preview() {
    //TODO: spawn when bomb held is requested, despawn when bomb throw requested
}
