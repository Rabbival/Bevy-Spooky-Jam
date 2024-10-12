use crate::prelude::*;

pub mod color_change;
pub mod scale_change;
pub mod translation_change;

pub struct CustomAnimationPlugin;

impl Plugin for CustomAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            TranslationChangePlugin,
            ScaleChangePlugin,
            ColorChangePlugin,
        ));
    }
}
