use crate::prelude::*;

pub mod bomb_explosion_animation;
pub mod color_change;
pub mod consts;
pub mod frame_change;
pub mod frame_sequence;
pub mod scale_change;
pub mod translation_change;

pub struct CustomAnimationPlugin;

impl Plugin for CustomAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TranslationChangePlugin,
            ScaleChangePlugin,
            ColorChangePlugin,
            FrameChangePlugin,
            BombExplosionAnimationPlugin,
        ));
    }
}
