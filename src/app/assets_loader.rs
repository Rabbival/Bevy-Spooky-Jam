use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct SpritesAtlas {
    pub atlas_handle: Handle<TextureAtlasLayout>,
    pub image_handle: Handle<Image>,
    pub pumpkin_image_handle: Handle<Image>,
    pub floor_image_handle: Handle<Image>,
    pub bato_san_image_handle: Handle<Image>,
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct TextFonts {
    #[deref]
    pub kenny_blocks_handle: Handle<Font>,
    pub kenny_high_square_handle: Handle<Font>,
    pub kenny_pixel_handle: Handle<Font>,
}

#[derive(Resource, Default)]
pub struct MusicAssets {
    pub calm_layer_handle: Handle<AudioSource>,
    pub intense_layer_handle: Handle<AudioSource>,
}

#[derive(Resource, Default)]
pub struct SoundAssets {
    pub bomb_explode: Handle<AudioSource>,
    pub bomb_pick_up: Handle<AudioSource>,
    pub bomb_throw: Handle<AudioSource>,
    pub bomb_tick: Handle<AudioSource>,
    pub monster_battle_cry: Handle<AudioSource>,
    pub monster_death_cry: Handle<AudioSource>,
}

pub struct AssetsLoaderPlugin;

impl Plugin for AssetsLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreStartup,
            (sprites_atlas_setup, text_font_setup, music_setup, sound_fx_setup),
        );
    }
}

fn sprites_atlas_setup(
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut commands: Commands,
) {
    let image_handle = asset_server.load("images/sprites_sheet.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(180, 101), 3, 3, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpritesAtlas {
        atlas_handle: texture_atlas_handle,
        image_handle,
        pumpkin_image_handle: asset_server.load("images/pumpkin.png"),
        floor_image_handle: asset_server.load("images/full_floor.png"),
        bato_san_image_handle: asset_server.load("images/bato_san.png"),
    });
}

fn text_font_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(TextFonts {
        kenny_blocks_handle: asset_server.load("fonts/kenney_blocks.ttf"),
        kenny_high_square_handle: asset_server.load("fonts/kenney_high_square.ttf"),
        kenny_pixel_handle: asset_server.load("fonts/kenney_pixel.ttf"),
    });
}

fn music_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(MusicAssets {
        calm_layer_handle: asset_server.load("music/music_calm_layer.ogg"),
        intense_layer_handle: asset_server.load("music/music_intense_layer.ogg"),
    });
}

fn sound_fx_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SoundAssets {
        bomb_explode: asset_server.load("sound_fx/bomb_explode.ogg"),
        bomb_pick_up: asset_server.load("sound_fx/bomb_pick_up.ogg"),
        bomb_throw: asset_server.load("sound_fx/bomb_throw.ogg"),
        bomb_tick: asset_server.load("sound_fx/bomb_tick.ogg"),
        monster_battle_cry: asset_server.load("sound_fx/monster_battle_cry.ogg"),
        monster_death_cry: asset_server.load("sound_fx/monster_death_cry.ogg"),
    });
}
