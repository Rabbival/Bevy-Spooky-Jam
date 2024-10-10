use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SpritesAtlas {
    pub atlas_handle: Handle<TextureAtlasLayout>,
    pub image_handle: Handle<Image>,
    pub pumpkin_image_handle: Handle<Image>,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct TextFonts {
    #[deref]
    pub kenny_blocks_handle: Handle<Font>,
    pub kenny_high_square_handle: Handle<Font>,
    pub kenny_pixel_handle: Handle<Font>,
}

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, (sprites_atlas_setup, text_font_setup));
    }
}

fn sprites_atlas_setup(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let image_handle = asset_server.load("images/sprites_sheet.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(40, 40), 3, 1, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpritesAtlas {
        atlas_handle: texture_atlas_handle,
        image_handle,
        pumpkin_image_handle: asset_server.load("images/pumpkin.png"),
    });
}

fn text_font_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(TextFonts {
        kenny_blocks_handle: asset_server.load("fonts/kenney_blocks.ttf"),
        kenny_high_square_handle: asset_server.load("fonts/kenney_high_square.ttf"),
        kenny_pixel_handle: asset_server.load("fonts/kenney_pixel.ttf"),
    });
}
