use crate::prelude::*;

pub struct FloorSpawningPlugin;

impl Plugin for FloorSpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_floor);
    }
}

fn spawn_floor(image_fonts_resource: ResMut<StaticImageHandles>, mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            texture: image_fonts_resource.floor_image_handle.clone(),
            ..default()
        },
        DoNotDestroyOnRestart,
    ));
}
